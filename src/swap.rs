use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256, address, utils::format_units},
    providers::{self, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;
use crate::shared::{get_provider, get_wallet, Token, get_token_list};

use crate::get_balance::{get_token_balance};

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
    #[sol(rpc)]
    contract Multicall3 {
        struct Call3 {
            address target;
            bool allowFailure;
            bytes callData;
        }

        struct Result {
            bool success;
            bytes returnData;
        }

        function aggregate3(Call3[] calldata calls)
            external payable
            returns (Result[] memory returnData);
    }
}


pub async fn swap_all(token_out: Token)-> Result<String>{
    let providers = get_provider().await?;
    let wallet = get_wallet().await?;

    
    Ok("".to_string())
}
pub async fn swap(token_in: Token, token_out: Token) -> Result<String> {
    dotenv().ok();

    // ========= Fetch Wallet and Router ========

    let wallet = get_wallet().await?; // TODO: Take wallet as an argument instead of fetching it here
    let wallet_address = wallet.default_signer().address();

    let router_address = address!("0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3");
    let router_provider = get_provider().await?;
    let router = IUniswapV2Router02::new(router_address, router_provider);

    // Get current balance
    let token_balance = get_token_balance(&token_in, wallet_address).await?;

    // ========= Token Approval ========

    let provider_for_approve = get_provider().await?;
    let token_contract = IERC20::new(token_in.address, provider_for_approve);

    let approve_tx_builder = token_contract
        .approve(router_address, token_balance);

    let approve_tx = approve_tx_builder.send()
        .await?;
    let _ = approve_tx.get_receipt().await?;

    // ========= Swap ========

    let path = vec![token_in.address, token_out.address];
    let deadline = U256::from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
            + 1200,
    );

    let swap_tx = router
        .swapExactTokensForTokens(
            token_balance,
            U256::from(0), // slippage tolerance
            path,
            wallet_address,
            deadline,
        )
        .send()
        .await?;

    let receipt = swap_tx.get_receipt().await?;
    println!("Swap successful! TX Hash: {:?}", receipt.transaction_hash);

    Ok(receipt.transaction_hash.to_string())
}
