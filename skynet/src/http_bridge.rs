static IROH_ENDPOINT: tokio::sync::OnceCell<iroh::Endpoint> = tokio::sync::OnceCell::const_new();

pub async fn http_bridge(
    port: u16,
    mut graceful_shutdown_rx: tokio::sync::watch::Receiver<bool>,
    proxy_target: Option<String>,
) -> eyre::Result<()> {
    use eyre::WrapErr;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .wrap_err_with(
            || "can not listen to port 80, is it busy, or you do not have root access?",
        )?;

    println!("Listening on http://127.0.0.1:{port}");

    let peer_connections = ftnet_utils::PeerConnections::default();

    loop {
        tokio::select! {
            _ = graceful_shutdown_rx.changed() => {
                tracing::info!("Stopping control server.");
                break;
            }
            val = listener.accept() => {
                let self_endpoint = IROH_ENDPOINT.get_or_init(new_iroh_endpoint).await.clone();
                let graceful_shutdown_rx = graceful_shutdown_rx.clone();
                let peer_connections = peer_connections.clone();
                let proxy_target = proxy_target.clone();
                match val {
                    Ok((stream, _addr)) => {
                        tokio::spawn(async move { handle_connection(self_endpoint, stream, graceful_shutdown_rx, peer_connections, proxy_target).await });
                    },
                    Err(e) => {
                        tracing::error!("failed to accept: {e:?}");
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_connection(
    self_endpoint: iroh::Endpoint,
    stream: tokio::net::TcpStream,
    mut graceful_shutdown_rx: tokio::sync::watch::Receiver<bool>,
    peer_connections: ftnet_utils::PeerConnections,
    proxy_target: Option<String>,
) {
    let io = hyper_util::rt::TokioIo::new(stream);

    let builder =
        hyper_util::server::conn::auto::Builder::new(hyper_util::rt::tokio::TokioExecutor::new());
    // the following builder runs only http2 service, whereas the hyper_util auto Builder runs an
    // http1.1 server that upgrades to http2 if the client requests.
    // let builder = hyper::server::conn::http2::Builder::new(hyper_util::rt::tokio::TokioExecutor::new());
    tokio::pin! {
        let conn = builder
            .serve_connection(
                io,
                hyper::service::service_fn(|r| handle_request(r, self_endpoint.clone(), peer_connections.clone(), proxy_target.clone())),
            );
    }

    if let Err(e) = tokio::select! {
        _ = graceful_shutdown_rx.changed() => {
            conn.as_mut().graceful_shutdown();
            conn.await
        }
        r = &mut conn => r,
    } {
        tracing::error!("connection error1: {e:?}");
    }
}

async fn handle_request(
    r: hyper::Request<hyper::body::Incoming>,
    self_endpoint: iroh::Endpoint,
    peer_connections: ftnet_utils::PeerConnections,
    proxy_target: Option<String>,
) -> ftnet_utils::http::ProxyResult {
    let peer_id = match r
        .headers()
        .get("Host")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.split_once('.'))
    {
        Some((first, _)) => {
            if first.len() != 52 {
                tracing::error!(peer_id = %first, "request received for invalid peer id");
                return Ok(ftnet_utils::bad_request!(
                    "got http request with invalid peer id"
                ));
            }

            if let Some(target) = proxy_target {
                if first != target {
                    tracing::error!(peer_id = %first, proxy_target = %target, "request for peer_id is not allowed");
                    return Ok(ftnet_utils::bad_request!(
                        "got http request with invalid peer id"
                    ));
                }
            }

            first.to_string()
        }
        None => {
            tracing::error!("got http request without Host header");
            return Ok(ftnet_utils::bad_request!(
                "got http request without Host header"
            ));
        }
    };

    tracing::info!("got request for {peer_id}");

    ftnet_utils::proxy::peer_to_peer(
        r,
        self_endpoint,
        &peer_id,
        peer_connections,
        Default::default(), /* RequestPatch */
    )
    .await
}

async fn new_iroh_endpoint() -> iroh::Endpoint {
    // TODO: read secret key from ENV VAR
    iroh::Endpoint::builder()
        .discovery_n0()
        .discovery_local_network()
        .alpns(vec![ftnet_utils::APNS_IDENTITY.into()])
        .bind()
        .await
        .expect("failed to create iroh Endpoint")
}
