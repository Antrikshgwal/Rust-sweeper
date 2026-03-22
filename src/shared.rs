use alloy::{
    network::EthereumWallet,
    primitives::{address, Address, utils::format_units, U256},
    providers::{ProviderBuilder, Provider},
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct Token {
    pub name: String,
    pub address: Address,
    pub decimals: u8,
}

pub async fn get_provider() -> Result<impl Provider> {
    dotenv().ok();
    let rpc_url = env::var("SEPOLIA_RPC_URL")?.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    Ok(provider)

}

pub async fn get_wallet() -> Result<EthereumWallet> {
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer);
    Ok(wallet)
}

pub fn get_token_list() -> Result<Vec<Token>> {
let mut token_list: Vec<Token> = Vec::new();
token_list.push(Token {
    name: String::from("USDC"),
    address: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".parse().unwrap(),
    decimals: 6,
});
token_list.push(Token {
    name: String::from("USDT"),
    address: "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06".parse().unwrap(),
    decimals: 6,
});
token_list.push(Token {
    name: String::from("WETH"),
    address: "0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9".parse().unwrap(),
    decimals: 18,
});
Ok(token_list)
}


