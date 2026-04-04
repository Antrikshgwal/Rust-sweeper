# Dust Sweeper

A blazing fast Rust-based dust sweeper for Ethereum wallets.

Scan wallets for small ERC-20 balances (dust) and sweep them into a single token (USDC, USDT, or WETH) — powered by Uniswap V2 on Sepolia testnet.

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/Antrikshgwal/Rust-sweeper.git
   cd Rust-sweeper
   ```
2. Create a `.env` file:
   ```
   SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/<YOUR_KEY>
   PRIVATE_KEY=<YOUR_PRIVATE_KEY>
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Frontend (Localhost)

DS has a frontend (not deployed yet), which shows an animation of sweeping while your tokens are actually being swept from your wallet.
<video controls src="20260404-1631-14.3900412.mp4" title="Title"></video>
```bash
cd frontend
npm install
npm run dev
```

Start the backend API server in a separate terminal:
```bash
cargo run server
```

## CLI Usage

#### Scan a wallet
```
cargo run -- scan 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844
```

#### Sweep to USDT (defaults to USDC)
```
cargo run -- sweep <WALLET_ADDRESS> --to USDT
```

#### Swap tokens
```
cargo run -- swap --amount <AMOUNT> --from WETH --to USDC <WALLET_ADDRESS>
```

#### With custom chain (future)
```
cargo run -- --chain sepolia scan 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (Next.js)                  │
│                                                         │
│  Landing Page ──► Connect Wallet ──► Dashboard          │
│                       │                                 │
│           ┌───────────┼───────────┐                     │
│           │           │           │                     │
│     TokenBalances   Sweep       Swap                    │
│       (view)      (batch)     (single)                  │
│           │           │           │                     │
│           └───────────┼───────────┘                     │
│                       │                                 │
│              MetaMask (Sepolia)                          │
└────────────────────── │ ────────────────────────────────┘
                        │  HTTP (localhost:3001)
┌────────────────────── │ ────────────────────────────────┐
│                  Rust Backend (Axum)                     │
│                       │                                 │
│         ┌─────────────┼─────────────┐                   │
│         │             │             │                   │
│    POST /scan    POST /sweep   POST /swap               │
│    (balances)    (calldata)    (calldata)                │
│         │             │             │                   │
│         └─────────────┼─────────────┘                   │
│                       │                                 │
│               Alloy (RPC client)                        │
└────────────────────── │ ────────────────────────────────┘
                        │  JSON-RPC
┌────────────────────── │ ────────────────────────────────┐
│               Sepolia Testnet                           │
│                       │                                 │
│    ┌──────────────────┼──────────────────┐              │
│    │                  │                  │              │
│  ERC-20s        DustSweeper       Uniswap V2           │
│  (USDC,USDT,    (batch sweep)      Router              │
│   WETH)                          (token swaps)          │
└─────────────────────────────────────────────────────────┘
```

### Backend (`src/`)

| File | Role |
|------|------|
| `main.rs` | Entry point — routes to CLI or API server mode |
| `api.rs` | Axum HTTP server with `/scan`, `/swap`, `/sweep`, `/broadcast` endpoints |
| `cli.rs` | CLI interface (scan, sweep, swap subcommands via clap) |
| `swap.rs` | Uniswap V2 calldata generation, ERC-20 approvals, DustSweeper integration |
| `get_balance.rs` | ERC-20 `balanceOf` + native ETH balance lookups |
| `shared.rs` | Token list, RPC provider, signer utilities |

### Frontend (`frontend/src/`)

| Component | Role |
|-----------|------|
| `page.tsx` | Landing page (pre-connect) / Dashboard (post-connect) |
| `Header.tsx` | Wallet connect/disconnect, Sepolia badge |
| `TokenBalances.tsx` | Fetches and displays token balances via `/scan` |
| `Sweep.tsx` | Batch sweep UI — approvals + DustSweeper contract call |
| `Swap.tsx` | Single token swap UI — approval + Uniswap V2 Router call |
| `Broom.tsx` | 3D broom animation (GLB model via Three.js) during sweep |
| `WalletProvider.tsx` | Wallet state + Sepolia chain enforcement |

### Data Flow

1. **Connect** — MetaMask switches to Sepolia, frontend stores signer
2. **Scan** — Frontend calls `POST /scan` → backend queries ERC-20 balances via RPC → returns token list
3. **Sweep** — Frontend calls `POST /sweep` → backend checks allowances, builds DustSweeper calldata → frontend signs approvals + sweep tx via MetaMask
4. **Swap** — Frontend calls `POST /swap` → backend builds Uniswap V2 calldata → frontend signs approval + swap tx via MetaMask

### Contracts (Sepolia)

| Contract | Address |
|----------|---------|
| DustSweeper | [`0xC04722cA1000111DB683e26b296C9CBEF8ED25E4`](https://sepolia.etherscan.io/address/0xC04722cA1000111DB683e26b296C9CBEF8ED25E4) |
| Uniswap V2 Router | [`0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3`](https://sepolia.etherscan.io/address/0xeE567Fe1712Faf6149d80dA1E6934E354124CfE3) |
| USDC | [`0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238`](https://sepolia.etherscan.io/address/0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238) |
| USDT | [`0x7169D38820dfd117C3FA1f22a697dBA58d90BA06`](https://sepolia.etherscan.io/address/0x7169D38820dfd117C3FA1f22a697dBA58d90BA06) |
| WETH | [`0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9`](https://sepolia.etherscan.io/address/0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9) |

---

Made with fun by AG.
