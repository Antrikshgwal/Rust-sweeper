use clap::{Parser};

mod cli;
mod get_balance;
mod shared;
mod swap;
mod api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
     let args: Vec<String> = std::env::args().collect();

    // Check if running in server mode
    if args.len() > 1 && args[1] == "server" {
        println!(" Starting Dust Sweeper API server...\n");
        api::start_server().await?;
        return Ok(());
    }

    let cli = cli::Cli::parse();

    if let Err(e) = cli::run_cli(cli).await {
        eprintln!("Error: {:?}", e);
    }
    Ok(())
}
