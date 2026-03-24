use crate::shared::{Token, get_provider, get_rpc_url, get_signer};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256, address},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use eyre::Result;

use crate::get_balance::get_wallet_balance;

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
pub async fn get_swap_all_calldata(
    user_addr: Address,
    target_token: Token,
) -> Result<(Address, Bytes)> {
    let sweeper_address = address!("0xC04722cA1000111DB683e26b296C9CBEF8ED25E4"); // Deployed Sweeper contract on Sepolia

    let dust_tokens = get_wallet_balance(user_addr)
        .await?
        .into_iter()
        .filter(|(token, balance)| *balance > U256::ZERO && token.address != target_token.address)
        .map(|(token, balance)| (token.address, balance))
        .collect::<Vec<(Address, U256)>>();

    let tokens_to_sweep: Vec<Address> = dust_tokens.iter().map(|(addr, _)| *addr).collect();
    let amounts_to_sweep: Vec<U256> = dust_tokens.iter().map(|(_, balance)| *balance).collect();

    let provider = get_provider().await?;
    let sweeper = DustSweeper::new(sweeper_address, provider);
    let calldata = sweeper
        .sweep(target_token.address, tokens_to_sweep, amounts_to_sweep)
        .calldata()
        .clone();

    Ok((sweeper_address, calldata))
}

pub async fn get_swap_calldata(
    wallet_address: Address,
    token_balance: U256,
    token_in: Token,
    token_out: Token,
) -> Result<(Address, Bytes)> {
    dotenv().ok();

    let router_address = address!("0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3");
    let provider = get_provider().await?;
    let router = IUniswapV2Router02::new(router_address, provider);

    let path = vec![token_in.address, token_out.address];
    let deadline = U256::from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
            + 1200,
    );

    let calldata = router
        .swapExactTokensForTokens(
            token_balance,
            U256::from(0), // slippage tolerance
            path,
            wallet_address,
            deadline,
        )
        .calldata()
        .clone();

    Ok((router_address, calldata))
}

pub async fn broadcast_transaction(signed_tx: Bytes) -> Result<String> {
    let rpc_url = get_rpc_url()?;
    let signer: PrivateKeySigner = get_signer()?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);
    let tx_hash = provider.send_raw_transaction(&signed_tx).await?.get_receipt().await?;
    Ok(tx_hash.transaction_hash.to_string())
}
