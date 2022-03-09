use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::{Pubkey}, example_mocks::solana_sdk::signature::Keypair};

fn main() {
    check_balance("https://api.devnet.solana.com", "5Z6FPt4C247g1cUDbeA5mphRxGmmupgwcnaJvFJLCXGn");
    // request_air_drop("https://api.devnet.solana.com");
}

fn check_balance(url: &str, public_key: &str) {
    let client = RpcClient::new(url);

    if let Ok(pubkey) = Pubkey::from_str(public_key) {
        let airdrop_res = client.request_airdrop(&pubkey, 10000000000);

        let balance_result = client.get_balance(&pubkey);

        match balance_result {
            Ok(balance) => println!("Balance of {:?} is {:?}", pubkey, balance),
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Invalid public key");
    }
}

fn request_air_drop(url: &str) {
    let client = RpcClient::new(url);

    let key_pair = Keypair::new();
    
    let mut balance_result = client.get_balance(&key_pair.pubkey());

    match balance_result {
        Ok(balance) => println!("Balance of {:?} is {:?}", key_pair.pubkey(), balance),
        Err(err) => println!("Error: {:?}", err),
    }

    let airdrop_res = client.request_airdrop(&key_pair.pubkey(), 10000000);

    match airdrop_res {
        Ok(sig) => {
            println!("Airdrop executed. Signature: {:?}", sig);

            balance_result = client.get_balance(&key_pair.pubkey());

            match balance_result {
                Ok(balance) => println!("Balance of {:?} is {:?}", key_pair.pubkey(), balance),
                Err(err) => println!("Error: {:?}", err),
            }
        },
        Err(err) => println!("Error: {:?}", err),
    }
}