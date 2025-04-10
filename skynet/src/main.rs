#[tokio::main]
async fn main() -> eyre::Result<()> {
    use clap::Parser;

    // run with RUST_LOG="skynet=info" to only see our logs when running with the --trace flag
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        Command::ExposeHttp {
            port,
            host,
            // secure,
            // what_to_do,
        } => {
            tracing::info!(port, host, verbose = ?cli.verbose, "Exposing HTTP service on FTNet.");
            skynet::expose_http(host, port).await
        }
        Command::HttpBridge { proxy_target, port } => {
            tracing::info!(port, proxy_target, verbose = ?cli.verbose, "Starting HTTP bridge.");
            skynet::http_bridge(proxy_target, port).await
        }
        Command::ExposeTcp { port, host } => {
            tracing::info!(port, host, verbose = ?cli.verbose, "Exposing TCP service on FTNet.");
            skynet::expose_tcp(host, port).await
        }
        t => {
            tracing::error!("Unsupported command: {t:?}");
            return Err(eyre::eyre!("Unsupported command: {t:?}"));
        }
    } {
        tracing::error!("Error: {e}");
        return Err(e);
    }

    Ok(())
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[clap(
        about = "Expose HTTP Service on FTNet, connect using FTNet.",
        long_about = r#"
Expose HTTP Service on FTNet, connect using FTNet.

By default it allows any peer to connecto to the HTTP(s) service. You can pass --what-to-do
argument to specify a What To Do service that can be used to add access control."#
    )]
    ExposeHttp {
        port: u16,
        #[arg(
            long,
            default_value = "127.0.0.1",
            help = "Host serving the http service."
        )]
        host: String,
        // #[arg(
        //     long,
        //     default_value_t = false,
        //     help = "Use this if the service is HTTPS"
        // )]
        // secure: bool,
        // #[arg(
        //     long,
        //     help = "The What To Do Service that can be used to add access control."
        // )]
        // this will be the id52 of the identity server that should be consulted
        // what_to_do: Option<String>,
    },
    HttpBridge {
        #[arg(
            long,
            short('t'),
            help = "The id52 to which this bridge will forward incoming HTTP request. By default it forwards to every id52."
        )]
        proxy_target: Option<String>,
        #[arg(
            long,
            short('p'),
            help = "The port on which this bridge will listen for incoming HTTP requests.",
            default_value = "8080"
        )]
        port: u16,
    },
    ExposeTcp {
        port: u16,
        #[arg(
            long,
            default_value = "127.0.0.1",
            help = "Host serving the TCP service."
        )]
        host: String,
    },
    TcpBridge {
        #[arg(help = "The id52 to which this bridge will forward incoming TCP request.")]
        proxy_target: String,
        #[arg(
            help = "The port on which this bridge will listen for incoming TCP requests.",
            default_value = "8081"
        )]
        port: u16,
    },
}
