# Q3 2025 Builders Qualifier

This repository contains implementations for the Q3 2025 Builders Qualifier prereq, featuring both Rust and TypeScript solutions for Solana blockchain interactions.

## 🏗️ Project Structure

Q3_2025_Builders_Qualifier/
├── airdrop_rs/ # Rust implementation
│ ├── src/
│ │ └── lib.rs # Main Rust library with test functions
│ ├── Cargo.toml # Rust dependencies
│ └── target/ # Build artifacts (ignored by git)
├── airdrop_ts/ # TypeScript implementation
│ ├── programs/
│ │ └── Turbin3_prereq.ts # Turbin3 prerequisite program
│ ├── *.ts # Various TypeScript scripts
│ ├── package.json # Node.js dependencies
│ └── node_modules/ # Dependencies (ignored by git)
└── .gitignore # Git ignore patterns


## 🚀 Features

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

## 📋 Prerequisites

### Rust
- [Rust](https://rustup.rs/) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/)

### TypeScript
- [Node.js](https://nodejs.org/) (v16+)
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/)

## 🛠️ Setup

### Rust Setup
```bash
cd airdrop_rs
cargo build

TypeScript Setup

cd airdrop_ts
yarn install
# or
npm install

🎯 Usage
cargo test keygen
cargo test base58_to_wallet
cargo test wallet_to_base58
cargo test claim_airdrop
cargo test transfer_sol
cargo test transfer_all
cargo test submit_rs

TypeScript
yarn keygen
yarn airdrop
yarn transfer
yarn transfer-all
yarn enroll
yarn convert

🔧 Configuration
Wallet Files (Not included in repo)
dev-wallet.json - Devnet wallet

Turbin3-wallet.json - Wallet for program submission

🌐 Network Info
Network: Solana Devnet

Rust RPC: https://turbine-solanad-4cde.devnet.rpcpool.com/

TypeScript RPC: https://api.devnet.solana.com

📦 Dependencies
Rust
solana-sdk, solana-client, bs58, borsh, solana-idlgen

TypeScript
@solana/web3.js, @coral-xyz/anchor, bs58, prompt-sync, typescript

🤝 Contributing
Fork this repo

Create a branch

Make changes

Test both implementations

Submit a PR

📄 License
This project is part of the Q3 2025 Builders Qualifier challenge.

🔗 Resources
Solana Docs

Anchor Lang

Solana Devnet Info

Turbin3 Program


Note: This repository is for educational and qualification purposes. Use only on Solana Devnet unless explicitly instructed.



