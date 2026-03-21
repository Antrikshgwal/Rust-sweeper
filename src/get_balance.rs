use alloy::{
    primitives::{address, Address, utils::{format_ether, format_units}},
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::Result;
use std::env;
use dotenv::dotenv;

sol!{
    #[sol(rpc)]
    contract ERC20{
    function balanceOf(address owner) public view returns (uint256){}
    }
}

pub struct Token {
    pub name: String,
    pub address: Address,
    pub decimals: u8,
}
pub fn get_token_list() -> Result<Vec<Token>> {
let mut token_list: Vec<Token> = Vec::new();
token_list.push(Token {
    name: String::from("ETH"),
    address: "0x0000000000000000000000000000000000000000".parse().unwrap(),
    decimals: 18,
});
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


pub async fn get_wallet_balance() -> Result<Vec<(String, u8)>> {

    let target_address = address!("0xfEfE12bf26A2802ABEe59393B19b0704Fb274844");
let token_list = get_token_list().unwrap();
    let mut balances: Vec<(String, u8)> = Vec::new();
    for token in token_list.iter() {
        let balance = get_token_balance(token, target_address).await.unwrap();
        balances.push((token.name.clone(), balance));
    }
    Ok(balances)
}

pub async fn get_token_balance(token: &Token, target_address: Address) -> Result<u8> {
dotenv().ok();
   let rpc_url = env::var("SEPOLIA_RPC_URL")?.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

 let token_address: Address = token.address;
        let contract = ERC20::new(token_address, provider.clone());
        let balance = format_units(contract.balanceOf(target_address).call().await?, token.decimals)?;
Ok(balance.parse::<u8>()?)
}
