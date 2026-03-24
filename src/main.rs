mod get_balance;
mod swap;
mod shared;
mod cli;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::CLI() {
        eprintln!("Error: {}", e);
    }
}