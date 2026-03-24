use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;

#[derive(Clone)]
pub struct Token {
    pub name: String,
    pub address: Address,
    pub decimals: u8,
}

pub fn get_rpc_url() -> Result<url::Url> {
    dotenv().ok();
    let rpc_url = env::var("SEPOLIA_RPC_URL")?.parse()?;
    Ok(rpc_url)
}

pub async fn get_provider() -> Result<impl Provider> {
    let rpc_url = get_rpc_url()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    Ok(provider)
}

pub fn get_signer() -> Result<PrivateKeySigner> {
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    Ok(signer)
}

pub fn get_token_list() -> Result<Vec<Token>> {
    let mut token_list: Vec<Token> = Vec::new();
    token_list.push(Token {
        name: String::from("USDC"),
        address: "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"
            .parse()
            .unwrap(),
        decimals: 6,
    });
    token_list.push(Token {
        name: String::from("USDT"),
        address: "0x7169D38820dfd117C3FA1f22a697dBA58d90BA06"
            .parse()
            .unwrap(),
        decimals: 6,
    });
    token_list.push(Token {
        name: String::from("WETH"),
        address: "0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9"
            .parse()
            .unwrap(),
        decimals: 18,
    });
    Ok(token_list)
}
