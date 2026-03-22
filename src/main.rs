mod get_balance;
mod swap;
mod shared;

use crate::shared::Token;
use alloy::primitives::utils::format_units;
use crate::swap::swap;
use crate::get_balance::get_wallet_balance;
use crate::get_balance::get_token_balance;
use alloy::primitives::address;

#[tokio::main]
async fn main() {
    let token_out: Token = Token {
        name: String::from("USDC"),
        address: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".parse().unwrap(),
        decimals: 6,
    };
    let token_in: Token = Token {
        name: String::from("USDT"),
        address: "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06".parse().unwrap(),
        decimals: 6,
    };
    match get_token_balance(&token_in, address!("0xfEfE12bf26A2802ABEe59393B19b0704Fb274844")).await {
        Ok(balance) => {
            println!("Balance of {}: {}", token_in.name, balance);
        }
        Err(e) => {
            eprintln!("Error fetching balance: {:?}", e);
        }
    }
    match get_wallet_balance(address!("0xfEfE12bf26A2802ABEe59393B19b0704Fb274844")).await {
        Ok(balances) => {
            println!("Wallet balances:");
            for (token, balance) in balances {
                println!("{}: {:?}", token.name, format_units(balance, token.decimals));
            }
        }
        Err(e) => {
            eprintln!("Error fetching wallet balances: {:?}", e);
        }
    }
    match swap(token_in, token_out).await {
        Ok(_) => println!("Swap successful!"),
        Err(e) => eprintln!("Error during swap: {:?}", e),
    }
}