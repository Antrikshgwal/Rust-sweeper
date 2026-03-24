use clap::{Parser, Subcommand};
use crate::get_balance::get_wallet_balance;
use crate::swap::{swap, swap_all};
use crate::shared::{Token, get_token_list, get_default};
use alloy::primitives::{Address, U256};

#[derive(Parser)]
#[command(name = "dust-sweep")]
#[command(about = "Sweep token dust into a single token", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan wallet for token balances
    Scan,
    /// Sweep all dust tokens into target token
    Sweep {
        /// Target token symbol (e.g., USDC, USDT, WETH)
        #[arg(short, long)]
        to: Option<String>,
    },
    Swap {
        /// Token symbol to swap from (e.g., WETH, USDT)
        #[arg(short, long)]
        from: Option<String>,
        /// Token symbol to swap to (e.g., USDC)
        #[arg(short, long)]
        to: Option<String>,
    },
}

// Helper function to find token by symbol
fn find_token_by_symbol(symbol: &str) -> eyre::Result<Token> {
    let tokens = get_token_list()?;
    tokens
        .into_iter()
        .find(|t| t.name.eq_ignore_ascii_case(symbol))
        .ok_or_else(|| eyre::eyre!("Token '{}' not found. Available: USDC, USDT, WETH", symbol))
}

#[tokio::main]
pub async fn CLI() -> eyre::Result<()> {
    let cli = Cli::parse();
    let wallet = crate::shared::get_wallet().await?;
    let wallet_address = wallet.default_signer().address();

    match cli.command {
        Commands::Scan => {
            let balances = get_wallet_balance(wallet_address).await?;
            println!("Token Balances:");
            for (token, balance) in balances {
                if balance > U256::ZERO {
                    println!("  {}: {}", token.name, balance);
                }
            }
        }
        Commands::Swap { to, from } => {
            let token_in: Token = match from {
                Some(symbol) => find_token_by_symbol(&symbol)?,
                None => get_default().await?,
            };

            let token_out: Token = match to {
                Some(symbol) => find_token_by_symbol(&symbol)?,
                None => get_default().await?,
            };

            swap(token_in, token_out).await?;
        }
        Commands::Sweep { to } => {
            let target = match to {
                Some(symbol) => find_token_by_symbol(&symbol)?,
                None => get_default().await?,
            };

            swap_all(target).await?;
        }
    }

    Ok(())
}