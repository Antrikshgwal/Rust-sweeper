use alloy::{
    primitives::{Address, U256, address},
    sol,
};
use dotenv::dotenv;
use eyre::Result;
use crate::shared::{Token, get_provider};

use crate::get_balance::{get_wallet_balance};

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
        function allowance(address owner, address spender) external view returns (uint256);
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

     #[sol(rpc)]
    contract DustSweeper {
        function sweep(address target, address[] calldata tokens, uint256[] calldata amounts) external;
    }
}
pub async fn swap_all(
    user_addr: Address,
    target_token: Token,
) -> Result<()> {
    // 1. Setup Provider with Signer

    let mut tokens_to_sweep = Vec::new();
    let mut amounts_to_sweep = Vec::new();
    let sweeper_address = address!("0xC04722cA1000111DB683e26b296C9CBEF8ED25E4"); // Deployed Sweeper contract on Sepolia

    let dust_tokens = get_wallet_balance(user_addr).await?
        .into_iter()
        .filter(|(token, balance)| {
            *balance > U256::ZERO && token.address != target_token.address
        })
        .map(|(token, balance)| (token.address, balance))
        .collect::<Vec<(Address, U256)>>();
    // 2. Check Allowances & Handle Approvals
    for (token_addr, balance) in dust_tokens {
        let provider = get_provider().await?;
        let token = IERC20::new(token_addr, provider);
        let current_allowance = token.allowance(user_addr, sweeper_address).call().await?;

        if current_allowance < balance {
            println!("Approving Sweeper for token {:?}", token_addr);
            // We use 'send' to broadcast; for better UX, you can wait for receipts in a batch
            token.approve(sweeper_address, U256::MAX).send().await?.get_receipt().await?;
        }

        tokens_to_sweep.push(token_addr);
        amounts_to_sweep.push(balance);
    }

    // 3. Execute Atomic Sweep
    let sweep_provider = get_provider().await?;
    if !tokens_to_sweep.is_empty() {
        println!("Executing bulk sweep for {} tokens...", tokens_to_sweep.len());
        let sweeper = DustSweeper::new(sweeper_address, sweep_provider);

        let tx = sweeper.sweep(target_token.address, tokens_to_sweep, amounts_to_sweep).send().await?;
        let receipt = tx.get_receipt().await?; // Wait for inclusion

        println!("Sweep successful! Tx Hash: {:?}", receipt.transaction_hash);
    }

    Ok(())
}


pub async fn swap(wallet_address: Address, token_balance: U256, token_in: Token, token_out: Token) -> Result<String> {
    dotenv().ok();

    // ========= Fetch Router =======

    let router_address = address!("0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3");
    let router_provider = get_provider().await?;
    let router = IUniswapV2Router02::new(router_address, router_provider);

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
