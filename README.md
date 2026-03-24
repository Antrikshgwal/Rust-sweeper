# DS
 A blazing fast Rust-based dust sweeper for Ethereum wallets

### Usage

#### Scan a wallet
`cargo run -- scan 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844`

#### Sweep wallet's dust to USDC
`cargo run -- sweep 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844`

#### Sweep to USDT
`cargo run -- sweep 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844 --to USDT`

#### Swap from one wallet
`cargo run -- swap 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844 --from WETH --to USDC`

#### With custom chain (future)
`cargo run -- --chain sepolia scan 0xfEfE12bf26A2802ABEe59393B19b0704Fb274844`

Sweeper contract address: [0xC04722cA1000111DB683e26b296C9CBEF8ED25E4](https://sepolia.etherscan.io/address/0xC04722cA1000111DB683e26b296C9CBEF8ED25E4)