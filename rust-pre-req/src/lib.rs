use bs58;
use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
use solana_program::{pubkey::Pubkey, system_instruction::transfer};

use solana_sdk::{
    blake3::hash,
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
    transaction::Transaction,
};

use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};
use std::io::{self, BufRead};
use std::str::FromStr;
mod programs;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}
#[test]
fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array:");
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
    println!("Your private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
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
fn airdrop() {
    let keypair = read_keypair_file("src/dev-wallet.json").expect("Incorrect wallet location");

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000) {
        Ok(s) => {
            println!("Airdropped successfully! Check txn here: ");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=devnet",
                s.to_string()
            );
        }
        Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
    }
}

#[test]
fn transfer_some_sol() {
    let keypair = read_keypair_file("src/dev-wallet.json").expect("Incorrect wallet location");

    let client = RpcClient::new(RPC_URL);

    let pubkey = keypair.pubkey();
    let message_bytes = b"I verified my solana keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified!"),
        false => println!("Verification of signature failed!"),
    }

    let to_pubkey = Pubkey::from_str("5P5z8bf9bmZvXaQoQB93pci4LuarBMKwa4Uhnzbjc3gG").unwrap();

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let txn = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&txn)
        .expect("Failed to send txn");

    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn transfer_all_sol() {
    let keypair = read_keypair_file("src/dev-wallet.json").expect("Incorrect wallet location");

    let client = RpcClient::new(RPC_URL);

    let pubkey = keypair.pubkey();

    let balance = client
        .get_balance(&pubkey)
        .expect("Failed to fetch balance");

    let to_pubkey = Pubkey::from_str("5P5z8bf9bmZvXaQoQB93pci4LuarBMKwa4Uhnzbjc3gG").unwrap();

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transfer_ix = &[transfer(&pubkey, &to_pubkey, balance)];

    let message = Message::new_with_blockhash(transfer_ix, Some(&pubkey), &recent_blockhash);

    let fee = client
        .get_fee_for_message(&message)
        .expect("Failed to get fee");

    let full_transfer_ix = &[transfer(&pubkey, &to_pubkey, balance - fee)];

    let txn = Transaction::new_signed_with_payer(
        full_transfer_ix,
        Some(&pubkey),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&txn)
        .expect("Failed to send txn");

    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn finalSubmission() {
    let signer = read_keypair_file("./src/turbine-wallet.json").expect("Couldn't find wallet file");

    let client = RpcClient::new(RPC_URL);
    let prereq = TurbinePrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref(),
    ]);
    let args = CompleteArgs {
        github: b"mrb1nary".to_vec(),
    };
    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = TurbinePrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
