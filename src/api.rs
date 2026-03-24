use axum::{
    extract::Json,
    http::Method,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use alloy::primitives::{Address, U256};
use eyre::Result;

use crate::{
    get_balance::get_wallet_balance,
    shared::{Token, get_token_list},
};
use crate::swap::{swap,swap_all};

#[derive(Deserialize)]
pub struct ScanRequest {
    wallet_address: String,
}
#[derive(Deserialize)]
pub struct SwapRequest {
    wallet_address: String,
    token_in: String,
    token_out: String,
    amount_in: String,
}
#[derive(Serialize)]
pub struct TokenBalance {
    name: String,
    address: String,
    balance: String,
    decimals: u8,
}

#[derive(Serialize)]
pub struct ScanResponse {
    balances: Vec<TokenBalance>,
}

#[derive(Serialize)]
pub struct SwapResponse {
    tx_receipt: String,
}

#[derive(Deserialize)]
pub struct SweepRequest {
    wallet_address: String,
    target_token: String,
}

#[derive(Serialize)]
pub struct SweepResponse {
    sweeper_address: String,
    target_token: String,
    tokens_to_sweep: Vec<String>,
    amounts_to_sweep: Vec<String>,
    approvals_needed: Vec<ApprovalNeeded>,
}

#[derive(Serialize)]
pub struct ApprovalNeeded {
    token_address: String,
    token_name: String,
    current_allowance: String,
    needed_amount: String,
}

// Scan endpoint - returns token balances
async fn scan_handler(
    Json(payload): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, String> {
    let wallet_address: Address = payload.wallet_address.parse()
        .map_err(|_| "Invalid wallet address".to_string())?;

    let balances = get_wallet_balance(wallet_address).await
        .map_err(|e| format!("Failed to get balances: {}", e))?;

    let token_balances: Vec<TokenBalance> = balances
        .into_iter()
        .map(|(token, balance)| TokenBalance {
            name: token.name.clone(),
            address: format!("{:?}", token.address),
            balance: balance.to_string(),
            decimals: token.decimals,
        })
        .collect();

    Ok(Json(ScanResponse {
        balances: token_balances,
    }))
}
async fn swap_handler(
    Json(payload): Json<SwapRequest>,
) -> Result<Json<SwapResponse>, String> {
    let wallet_address: Address = payload.wallet_address.parse()
        .map_err(|_| "Invalid wallet address".to_string())?;

    let tokens = get_token_list()
        .map_err(|e| format!("Failed to get token list: {}", e))?;

    let token_in = tokens
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(&payload.token_in))
        .ok_or_else(|| "Input token not found".to_string())?
        .clone();

    let token_out = tokens
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(&payload.token_out))
        .ok_or_else(|| "Output token not found".to_string())?
        .clone();

    let amount_in: U256 = payload.amount_in.parse()
        .map_err(|_| "Invalid amount".to_string())?;

    let tx_receipt = swap(wallet_address, amount_in, token_in.clone(), token_out.clone()).await
        .map_err(|e| format!("Swap failed: {}", e))?;

    Ok(Json(SwapResponse {
        tx_receipt: format!("{:?}", tx_receipt),
    }))
}
// Sweep endpoint - returns data needed for frontend to execute sweep
async fn sweep_handler(
    Json(payload): Json<SweepRequest>,
) -> Result<Json<SweepResponse>, String> {
    use crate::swap::IERC20;
    use crate::shared::get_provider;
    use alloy::primitives::address;

    let wallet_address: Address = payload.wallet_address.parse()
        .map_err(|_| "Invalid wallet address".to_string())?;

    let sweeper_address = address!("0xC04722cA1000111DB683e26b296C9CBEF8ED25E4");

    // Find target token
    let tokens = get_token_list()
        .map_err(|e| format!("Failed to get token list: {}", e))?;

    let target_token = tokens
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(&payload.target_token))
        .ok_or_else(|| "Token not found".to_string())?
        .clone();

    // Get dust tokens
    let dust_tokens = get_wallet_balance(wallet_address).await
        .map_err(|e| format!("Failed to get balances: {}", e))?
        .into_iter()
        .filter(|(token, balance)| {
            *balance > U256::ZERO && token.address != target_token.address
        })
        .collect::<Vec<(Token, U256)>>();

    let mut tokens_to_sweep = Vec::new();
    let mut amounts_to_sweep = Vec::new();
    let mut approvals_needed = Vec::new();

    // Check allowances
    for (token, balance) in dust_tokens {
        let provider = get_provider().await
            .map_err(|e| format!("Provider error: {}", e))?;
        let token_contract = IERC20::new(token.address, provider);

        let current_allowance = token_contract
            .allowance(wallet_address, sweeper_address)
            .call()
            .await
            .map_err(|e| format!("Allowance check failed: {}", e))?;

        if current_allowance < balance {
            approvals_needed.push(ApprovalNeeded {
                token_address: format!("{:?}", token.address),
                token_name: token.name.clone(),
                current_allowance: current_allowance.to_string(),
                needed_amount: balance.to_string(),
            });
        }

        tokens_to_sweep.push(format!("{:?}", token.address));
        amounts_to_sweep.push(balance.to_string());
    }

    Ok(Json(SweepResponse {
        sweeper_address: format!("{:?}", sweeper_address),
        target_token: format!("{:?}", target_token.address),
        tokens_to_sweep,
        amounts_to_sweep,
        approvals_needed,
    }))
}

pub async fn start_server() -> Result<()> {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/scan", post(scan_handler))
        .route("/sweep", post(sweep_handler))
        .route("/swap", post(swap_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("🚀 API Server running on http://localhost:3001");
    println!("📝 Endpoints:");
    println!("   GET  /health");
    println!("   POST /scan");
    println!("   POST /swap");
    println!("   POST /sweep");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}