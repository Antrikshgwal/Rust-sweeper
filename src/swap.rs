use alloy::{
    primitives::{address, Address, utils::{format_ether, format_units}},
    providers::{Provider, ProviderBuilder},
    sol,
};
use std::Error;

sol! {
    #[sol(rpc)]
    contract IUniswapV2Router {
        function swapExactETHForTokens(
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external payable returns (uint[] memory amounts);
    }
}

struct Token {
    name: String,
    address: Address,
    decimals: u8,
}

pub async fn swap_eth_to_weth() -> Result<()> {
    let rpc_url = env::var("SEPOLIA_RPC_URL")?.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Uniswap V2 Router on Sepolia
    let router_address: Address = "0xC532a74256D3Db42D0Bf7a0400fEFDbad7694008".parse()?;
    let router = IUniswapV2Router::new(router_address, provider);

    let weth_address: Address = "0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9".parse()?;
    let your_address: Address = "0xfEfE12bf26A2802ABEe59393B19b0704Fb274844".parse()?;

    // Path: ETH -> WETH (WETH is the wrapped version, so path is just [WETH])
    let path = vec![weth_address];

    let amount_out_min = U256::from(0); // Set slippage tolerance in production
    let deadline = U256::from(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() + 300); // 5 min from now


        let token_balance = 
    // This is where you'd actually send the tx
    router.swapExactETHForTokens(token_balance,amount_out_min, path, your_address, deadline)
        .value(U256::from(100000000000000000u64)) // 0.1 ETH
        .send()
        .await?;

    Ok(())
}