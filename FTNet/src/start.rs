/// start FTNet service
///
/// on startup, we first check if another instance is running if so we exit.
///
/// the main job of this function is to run an instance of `fastn` for every identity in the
/// identities folder, and set-up http device driver for each of them.
///
/// it also has to start the device "drivers" for every device in the <identities>/devices folder.
pub async fn start(_fg: bool, data_dir: std::path::PathBuf, control_port: u16) -> eyre::Result<()> {
    use eyre::WrapErr;

    let client_pools = ftnet::http::client::ConnectionPools::default();
    let peer_connections = ftnet::identity::PeerConnections::default();

    let config = ftnet::Config::read(&data_dir, client_pools.clone())
        .await
        .wrap_err_with(|| "failed to run config")?;

    let _lock = config
        .lock()
        .await
        .wrap_err_with(|| "looks like there is another instance of FTNet running")?;

    let identities = config.identities(client_pools.clone()).await?;
    println!(
        "FTNet started with {identities}.",
        identities = identities
            .iter()
            .map(|i| i.id52.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let (graceful_shutdown_tx, graceful_shutdown_rx) = tokio::sync::watch::channel(false);

    let first = identities
        .first()
        .map(|v| v.id52.clone())
        .ok_or_else(|| eyre::eyre!("no identities found"))?;

    let id_map = ftnet::identity::IDMap::default();

    for identity in identities {
        use std::sync::Arc;

        let graceful_shutdown_rx = graceful_shutdown_rx.clone();
        let id_map = Arc::clone(&id_map);
        let peer_connections = Arc::clone(&peer_connections);
        let data_dir = data_dir.clone();
        tokio::spawn(async move {
            let public_key = identity.public_key;
            if let Err(e) = identity
                .run(graceful_shutdown_rx, id_map, peer_connections, &data_dir)
                .await
            {
                eprintln!("failed to run identity: {public_key}: {e:?}");
            }
        });
    }

    tokio::spawn(async move {
        tracing::info!("Starting control server with identity: {first}");
        ftnet::control_server::start(
            control_port,
            first,
            graceful_shutdown_rx,
            id_map,
            client_pools,
            peer_connections,
        )
        .await
        .unwrap()
    });

    tokio::signal::ctrl_c()
        .await
        .wrap_err_with(|| "failed to get ctrl-c signal handler")?;

    graceful_shutdown_tx
        .send(true)
        .wrap_err_with(|| "failed to send graceful shutdown signal")?;

    let mut count = 0;

    loop {
        count += 1;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let v = ftnet::OPEN_CONTROL_CONNECTION_COUNT.get();
        if v == 0 {
            println!("No inflight requests open.");
            break;
        }

        // every second print status
        if count % 10 == 0 {
            println!("Waiting for {v} inflight requests to finish.");
        }

        // give up in 1 min
        if count > 60 {
            println!("Giving up.");
            break;
        }
    }

    println!("Shutting down.");

    Ok(())
}
