use alloy::{
    network::EthereumWallet,
    primitives::{address, Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use std::env;
use dotenv::dotenv;

use crate::get_balance::{get_token_balance, Token};

sol! {
    #[sol(rpc)]
    contract IUniswapV2Router02 {
        function swapExactTokensForTokens(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapExactETHForTokens(
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external payable returns (uint[] memory amounts);
    }

    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

pub async fn swap(token: Token, target_address: Address) -> Result<()> {
    dotenv().ok();

    let rpc_url = env::var("SEPOLIA_RPC_URL")?.parse()?;

    // Set up the signer and wallet from your private key
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer);

    // Create the provider with the wallet attached so we can sign transactions
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(rpc_url);

    // This is the standard Uniswap V2 Router address on Sepolia
    // You can update this if you're using a different deployment
    let router_address = address!("0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3");
    let router = IUniswapV2Router02::new(router_address, provider.clone());

    // Get current balance
    let token_balance = get_token_balance(&token, target_address).await?;
    println!("{} balance of {} is \"{}\"", token.name, target_address, token_balance);


    // Example swap logic (Dust sweep):

    // 1. Approve router to spend token
    let token_contract = IERC20::new(token.address, provider.clone());

    // Parse the string balance back to U256
    let amount_in = std::str::FromStr::from_str(&token_balance).unwrap_or(U256::ZERO);

    let approve_tx = token_contract.approve(router_address, amount_in).send().await?;
    let _ = approve_tx.get_receipt().await?;

    // 2. Execute swap
    let path = vec![token.address, target_address /* e.g., WETH address */];
    let deadline = U256::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() + 1200);

    let swap_tx = router.swapExactTokensForTokens(
        amount_in,
        U256::from(0), // slippage tolerance
        path,
        target_address,
        deadline
    ).send().await?;

    let receipt = swap_tx.get_receipt().await?;
    println!("Swap successful! TX Hash: {:?}", receipt.transaction_hash);


    Ok(())
}