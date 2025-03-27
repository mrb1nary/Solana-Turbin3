use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use std::io::{self, BufRead};


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
fn airdrop(){
    let keypair = read_keypair_file("src/dev-wallet.json").expect("Incorrect wallet location");
    

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(),2_000_000_000){
        Ok(s)=>{
            println!("Airdropped successfully! Check txn here: ");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        },
        Err(e) => println!("Oops, something went wrong: {}", e.to_string())
    
    }
}