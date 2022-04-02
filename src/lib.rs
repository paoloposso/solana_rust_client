use std::error::Error;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{system_transaction, signature::{Keypair, Signature}};

const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: f64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_funds(rpc_client: &RpcClient, sender_keypair: &Keypair, receiver_pub_key: &Pubkey, amount_sol: f64) 
        -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;
    
    Ok(rpc_client.send_and_confirm_transaction(
        &system_transaction::transfer(
            &sender_keypair, &receiver_pub_key, 
            amount_lamports, 
            rpc_client.get_latest_blockhash()?))?)
}
