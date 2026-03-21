mod get_balance;

use crate::get_balance::{get_wallet_balance, get_token_balance, Token};

#[tokio::main]
async fn main() {
    let token: Token = Token {
        name: String::from("WETH"),
        address: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".parse().unwrap(),
        decimals: 6,
    };
    match get_token_balance(&token, address!("0xfEfE12bf26A2802ABEe59393B19b0704Fb274844")).await {
        Ok(balance) => {
            println!("Balance of {}: {}", token.name, balance);
        }
        Err(e) => {
            eprintln!("Error fetching balance: {:?}", e);
        }
    }
}