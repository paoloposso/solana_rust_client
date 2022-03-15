use std::{str::FromStr, error::Error};

use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::{Pubkey}};
use solana_sdk::{commitment_config::CommitmentConfig, system_transaction, signature::{Keypair, Signature}, signer::Signer};

fn main() {
    // let rpc = RpcClient::new_with_commitment("https://api.devnet.solana.com", CommitmentConfig::confirmed());
    let rpc = RpcClient::new("https://api.devnet.solana.com");

    let sender = Keypair::new();
    let receiver = Keypair::new();
    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    let amount = 100000000;

    if let Ok(airdrop_signature) = request_air_drop(&rpc, &sender.pubkey(), 1000000000) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        let res = transfer_funds(&rpc, &sender, &receiver.pubkey(), amount);
        match res {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", amount, sig);
                if let Ok(balance) = check_balance(&rpc, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Airdrop failed");
    }
}

fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> core::result::Result<u64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)?)
}

fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_lamports: u64) -> core::result::Result<Signature, Box<dyn Error>> {
    Ok(rpc_client.request_airdrop(&pub_key, amount_lamports)?)
}

fn transfer_funds(rpc_client: &RpcClient, sender_keypair: &Keypair, recipient_pub_key: &Pubkey, amount_lamports: u64) -> core::result::Result<Signature, Box<dyn Error>> {
    Ok(rpc_client.send_transaction(&system_transaction::transfer(&sender_keypair, &recipient_pub_key, amount_lamports, rpc_client.get_latest_blockhash()?))?)
}