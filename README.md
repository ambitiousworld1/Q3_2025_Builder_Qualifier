# Q3 2025 Builders Qualifier

A comprehensive development suite implementing Solana blockchain functionalities in both Rust and TypeScript. Created for the Q3 2025 Builders Qualifier program to demonstrate proficiency in Solana ecosystem development.

## 📂 Architecture Overview

```
Q3_2025_Builders_Qualifier/
├── airdrop_rs/                    # Rust-based implementation
│   ├── src/
│   │   └── lib.rs                 # Core library with test suite
│   ├── Cargo.toml                 # Rust project manifest
│   └── target/                    # Build outputs (git-ignored)
├── airdrop_ts/                    # TypeScript-based implementation
│   ├── programs/
│   │   └── Turbin3_prereq.ts      # Turbin3 program interface
│   ├── *.ts                       # Implementation scripts
│   ├── package.json               # Node.js project configuration
│   └── node_modules/              # Package dependencies (git-ignored)
└── .gitignore                     # Version control exclusions
```


## 🌟 Implementation Features

### Rust Implementation (`airdrop_rs/`)
- ✅ Keypair generation & Base58 conversion
- 🚰 Airdrop SOL tokens from devnet
- 🔄 Transfer partial and full wallet balances
- 📤 Submit to the Turbin3 program

### TypeScript Implementation (`airdrop_ts/`)
- 🔑 Wallet generation & key conversion
- 🚰 Airdrop requests from Solana devnet
- 🔄 Transfer SOL to specific addresses or full balance
- 📝 Enroll in Turbin3 program and handle prerequisites

## ⚙️ Development Environment
### Rust
- [Rust](https://rustup.rs/) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/)

### TypeScript
- [Node.js](https://nodejs.org/) (v16+)
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/)

## 🔨 Build Instructions

### Rust Setup
```bash
cd airdrop_rs
cargo build

TypeScript Setup

cd airdrop_ts
yarn install
# or
npm install

## 🎯 Usage

### Rust Commands

cargo test keygen
cargo test base58_to_wallet
cargo test wallet_to_base58
cargo test claim_airdrop
cargo test transfer_sol
cargo test transfer_all
cargo test submit_rs

### TypeScript Commands
yarn keygen
yarn airdrop
yarn transfer
yarn transfer-all
yarn enroll
yarn convert

## 🔧 Configuration
### Wallet Files (Not included in repo)

dev-wallet.json - Devnet wallet

Turbin3-wallet.json - Wallet for program submission

## 🌐 Network Information

- **Network**: Solana Devnet
- **RPC URL (Rust)**: `https://turbine-solanad-4cde.devnet.rpcpool.com/`
- **RPC URL (TypeScript)**: `https://api.devnet.solana.com`
- **Explorer**: https://explorer.solana.com/?cluster=devnet

### Rust Dependencies
- `solana-sdk` - Solana SDK for Rust
- `solana-client` - Solana RPC client
- `solana-program` - Solana program framework
- `bs58` - Base58 encoding/decoding
- `borsh` - Binary serialization
- `solana-idlgen` - IDL generation

### TypeScript Dependencies
- `@solana/web3.js` - Solana Web3.js library
- `@coral-xyz/anchor` - Anchor framework
- `bs58` - Base58 encoding/decoding
- `prompt-sync` - Command line prompts
- `typescript` - TypeScript compiler

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test both Rust and TypeScript implementations
5. Submit a pull request

📄 License
This project is part of the Q3 2025 Builders Qualifier challenge.

## 🔗 Resources
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Devnet](https://docs.solana.com/clusters)
- [Turbin3 Program](https://explorer.solana.com/address/TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM?cluster=devnet)


Note: This repository is for educational and qualification purposes. Use only on Solana Devnet unless explicitly instructed.



