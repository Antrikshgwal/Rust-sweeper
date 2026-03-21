mod get_balance;
mod swap;

use crate::get_balance::Token;
use crate::swap::swap;
use alloy::primitives::address;

#[tokio::main]
async fn main() {
    let token: Token = Token {
        name: String::from("USDT"),
        address: "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06".parse().unwrap(),
        decimals: 6,
    };
    // match get_token_balance(&token, address!("0xfEfE12bf26A2802ABEe59393B19b0704Fb274844")).await {
    //     Ok(balance) => {
    //         println!("Balance of {}: {}", token.name, balance);
    //     }
    //     Err(e) => {
    //         eprintln!("Error fetching balance: {:?}", e);
    //     }
    // }
    match swap(token, address!("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238")).await {
        Ok(_) => println!("Swap successful!"),
        Err(e) => eprintln!("Error during swap: {:?}", e),
    }
}