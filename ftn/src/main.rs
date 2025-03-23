#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = ftn::Cli::parse();
    println!("{cli:?}");
    if let Err(e) = match cli.command {
        ftn::Command::Start {
            foreground,
            data_dir,
        } => ftn::start(foreground, data_dir).await,
        ftn::Command::TcpProxy { id, port } => {
            println!("Proxying TCP server to remote ftn service with id: {id}, port: {port}");
            Ok(())
        }
    } {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
