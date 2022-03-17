use std::{error::Error, str::FromStr};

use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::{Pubkey}};
use solana_sdk::{system_transaction, signature::{Keypair, Signature}, signer::Signer};

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc = RpcClient::new(URL);

    _ = print_basic_operations(&rpc);

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
    let sig = rpc_client.request_airdrop(&pub_key, amount_lamports)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

fn transfer_funds(rpc_client: &RpcClient, sender_keypair: &Keypair, recipient_pub_key: &Pubkey, amount_lamports: u64) -> core::result::Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.send_and_confirm_transaction(&system_transaction::transfer(&sender_keypair, &recipient_pub_key, amount_lamports, rpc_client.get_latest_blockhash()?))?;
    Ok(sig)
}

fn print_basic_operations(rpc_client: &RpcClient) -> Result<(), Box<dyn Error>> {
    let vec = rpc_client.get_program_accounts(&Pubkey::from_str("HAKGVjYFMfhsaTHMk215ZeVbfuYn1fJgwNsE9iJ24zZ9")?)?;
    println!("{:?}", vec);
    Ok(())
}