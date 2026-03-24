use clap::{Parser};

mod cli;
mod get_balance;
mod shared;
mod swap;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    if let Err(e) = cli::run_cli(cli).await {
        eprintln!("Error: {:?}", e);
    }
}
