#[tokio::main]
async fn main() -> eyre::Result<()> {
    use clap::Parser;

    let cli = ftnet::Cli::parse();
    match cli.command {
        ftnet::Command::Start {
            foreground,
            data_dir,
        } => ftnet::start(foreground, data_dir).await,
        ftnet::Command::TcpProxy { id, port } => {
            println!("Proxying TCP server to remote FTNet service with id: {id}, port: {port}");
            Ok(())
        }
    }
}
