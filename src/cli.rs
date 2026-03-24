use clap::{Parser, Subcommand};
use crate::get_balance::get_wallet_balance;
use crate::swap::{swap, swap_all};
use crate::shared::{Token, get_token_list};
use alloy::primitives::{Address, U256, utils::format_units};

#[derive(Parser)]
#[command(name = "dust-sweep")]
#[command(about = "Sweep token dust into a single token", long_about = None)]
pub struct Cli {
    /// Chain to use (currently only sepolia supported)
    #[arg(short, long, default_value = "sepolia")]
    pub chain: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan wallet for token balances
    Scan {
        /// Wallet address to scan
        #[arg(value_name = "WALLET_ADDRESS")]
        wallet_address: String,
    },

    /// Sweep all dust tokens into target token
    Sweep {
        /// Wallet address to sweep from
        #[arg(value_name = "WALLET_ADDRESS")]
        wallet_address: String,

        /// Target token symbol (e.g., USDC, USDT, WETH)
        #[arg(short, long, default_value = "USDC")]
        to: String,
    },

    /// Swap one token for another
    Swap {
        /// Wallet address to swap from
        #[arg(value_name = "WALLET_ADDRESS")]
        wallet_address: String,

        /// Token symbol to swap from (e.g., WETH, USDT)
        #[arg(short, long)]
        from: String,

        /// Token symbol to swap to (e.g., USDC)
        #[arg(short, long)]
        to: String,
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

pub async fn run_cli(cli: Cli) -> eyre::Result<()> {
    // Validate chain
    if cli.chain != "sepolia" {
        return Err(eyre::eyre!("Currently only 'sepolia' chain is supported"));
    }

    match cli.command {
        Commands::Scan { wallet_address } => {
            let addr: Address = wallet_address.parse()
                .map_err(|_| eyre::eyre!("Invalid wallet address: {}", wallet_address))?;

            println!("Scanning wallet: {}", addr);
            println!("Chain: {}\n", cli.chain);

            let balances = get_wallet_balance(addr).await?;
            println!("Token Balances:");
            for (token, balance) in balances {
                if balance > U256::ZERO {
                    let formatted = format_units(balance, token.decimals)?;
                    println!("  {}: {}", token.name, formatted);
                }
            }
        }

        Commands::Swap { wallet_address, from, to } => {
            let addr: Address = wallet_address.parse()
                .map_err(|_| eyre::eyre!("Invalid wallet address: {}", wallet_address))?;

            let token_in = find_token_by_symbol(&from)?;
            let token_out = find_token_by_symbol(&to)?;

            println!("Wallet: {}", addr);
            println!("Chain: {}", cli.chain);
            println!("Swapping {} → {}\n", token_in.name, token_out.name);

            swap(addr, token_in, token_out).await?;
        }

        Commands::Sweep { wallet_address, to } => {
            let addr: Address = wallet_address.parse()
                .map_err(|_| eyre::eyre!("Invalid wallet address: {}", wallet_address))?;

            let target = find_token_by_symbol(&to)?;

            println!("Wallet: {}", addr);
            println!("Chain: {}", cli.chain);
            println!("Sweeping all dust to {}\n", target.name);

            swap_all(addr, target).await?;
        }
    }

    Ok(())
}