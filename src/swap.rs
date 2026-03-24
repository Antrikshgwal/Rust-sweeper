use alloy::{
    primitives::{Address, U256, address, Bytes},
    sol,
    sol_types::SolCall
};
use dotenv::dotenv;
use eyre::Result;
use crate::shared::{get_provider, get_wallet, Token};

use crate::get_balance::{get_token_balance, get_wallet_balance};

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


pub async fn swap_all(wallet_address: Address, target_token: Token) -> Result<()> {
    dotenv().ok();

    let router_address = address!("0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3"); // Sepolia V2 Router
    let multicall_address = address!("0xcA11bde05977b3631167028862bE2a173976CA11"); // Multicall3

    // Get all token balances
    let balances = get_wallet_balance(wallet_address).await?;

    // Filter out zero balances and target token
    let dust_tokens: Vec<(Token, U256)> = balances
        .into_iter()
        .filter(|(token, balance)| {
            *balance > U256::ZERO && token.address != target_token.address
        })
        .collect();

    if dust_tokens.is_empty() {
        println!("No dust to sweep!");
        return Ok(());
    }

    println!("Found {} tokens to sweep:", dust_tokens.len());
    for (token, balance) in &dust_tokens {
        println!("  - {}: {}", token.name, balance);
    }
    let multicall_provider = get_provider().await?;
    let multicall = Multicall3::new(multicall_address, multicall_provider);
    let mut calls: Vec<Multicall3::Call3> = Vec::new();

    let deadline = U256::from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() + 1200
    );

    // Build approve + swap calls for each token
    for (token, balance) in dust_tokens {
        println!("\nPreparing {} swap...", token.name);

        // 1. Approve router to spend tokens
        let approve_call = IERC20::approveCall {
            spender: router_address,
            amount: balance,
        };
        let approve_calldata = approve_call.abi_encode();

        calls.push(Multicall3::Call3 {
            target: token.address,
            allowFailure: false,
            callData: Bytes::from(approve_calldata),
        });

        // 2. Swap tokens
        let path = vec![token.address, target_token.address];
        let swap_call = IUniswapV2Router02::swapExactTokensForTokensCall {
            amountIn: balance,
            amountOutMin: U256::ZERO, // TODO: Add slippage protection
            path,
            to: wallet_address,
            deadline,
        };
        let swap_calldata = swap_call.abi_encode();

        calls.push(Multicall3::Call3 {
            target: router_address,
            allowFailure: false,
            callData: Bytes::from(swap_calldata),
        });
    }

    println!("\n Executing {} operations in one transaction...", calls.len());

    // Execute all in ONE transaction
    let tx = multicall.aggregate3(calls).send().await?;
    let receipt = tx.get_receipt().await?;

    println!("\n Batched sweep complete!");
    println!(" Transaction: {:?}", receipt.transaction_hash);
    println!(" Gas used: {}", receipt.gas_used);

    Ok(())
}
pub async fn swap(wallet_address: Address, token_in: Token, token_out: Token) -> Result<String> {
    dotenv().ok();

    // ========= Fetch Router =======

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
