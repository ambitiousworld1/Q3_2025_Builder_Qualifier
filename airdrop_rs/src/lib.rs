use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::{hash::hash, pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    system_program
};
use solana_sdk::instruction::{AccountMeta, Instruction};
use std::io::{self, BufRead};
use std::str::FromStr;
const RPC_URL: &str =
        "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";


#[cfg(test)]
mod tests {
    //use solana_sdk;
    #[test]
    fn keygen() {}
    #[test]
    fn airdrop() {}
    #[test]
    fn transfer_sol() {}
    #[test]
    fn submit_rs(){}
}

#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

#[test]
fn base58_to_wallet() {
    println!("Input your private key as a base58 string:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file format is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Your Base58-encoded private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}

fn airdrop() {
    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(sig) => {
            println!("Success! Check your TX here:");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=devnet",
                sig.to_string()
            );
        }
        Err(err) => {
            println!("Airdrop failed: {}", err.to_string());
        }
    };
}

#[test]
fn transfer_sol() {
    // Load your devnet keypair from file
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Generate a signature from the keypair
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    // Verify the signature using the public key
    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }

    let to_pubkey = Pubkey::from_str("HCR3xpRDEQvjZW9Z3m8RvsBczEXnNhGUjRNH4GwoatDU").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn transfer_sol_all() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("HCR3xpRDEQvjZW9Z3m8RvsBczEXnNhGUjRNH4GwoatDU").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");

    println!(
        "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]

fn submit_rs() {
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

    let mint = Keypair::new();
    let turbin3_prereq_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    let signer_pubkey = signer.pubkey();
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
    let seeds_authority = &[b"collection", collection.as_ref()];
    let (authority, _bump) = Pubkey::find_program_address(seeds_authority, &turbin3_prereq_program);
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(prereq_pda, false),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new_readonly(mpl_core_program, false),
        AccountMeta::new_readonly(system_program, false),
    ];
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
