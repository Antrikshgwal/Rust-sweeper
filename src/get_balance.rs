use alloy::{
    primitives::{address, Address, U256},
    providers::{Provider},
    sol,
};
use eyre::Result;
use crate::shared::{Token, get_provider, get_token_list};

sol!{
    #[sol(rpc)]
    contract ERC20{
    function balanceOf(address owner) public view returns (uint256){}
    }
}

pub async fn get_wallet_balance(target_address: Address) -> Result<Vec<(Token, U256)>> {
    let token_list = get_token_list()?;

    let mut balances: Vec<(Token, U256)> = Vec::new();

    for token in token_list.iter() {
        match get_token_balance(token, target_address).await {
            Ok(balance) => {
                if balance > U256::ZERO {
                    balances.push((token.clone(), balance));
                }
            }
            Err(e) => {
                eprintln!("Failed to get balance for {}: {}", token.name, e);
                // Skip this token, continue with others
            }
        }
    }

    Ok(balances)
}

pub async fn get_token_balance(token: &Token, target_address: Address) -> Result<U256> {
    let provider = get_provider().await?;
    // Special case: ETH is not an ERC20 token
    if token.address == address!("0x0000000000000000000000000000000000000000") {
        let balance = provider.get_balance(target_address).await?;
        return Ok(balance);
    }

    // For actual ERC20 tokens
    let contract = ERC20::new(token.address, provider);
    let balance = contract.balanceOf(target_address).call().await?;

    Ok(balance)
}