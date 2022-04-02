use solana_client::rpc_client::RpcClient;
use solana_rust_client::{check_balance, request_air_drop, transfer_funds, create_keypair};
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc_client = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        let transfer_amount = 0.5;

        match transfer_funds(&rpc_client, &sender, &receiver.pubkey(), transfer_amount) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Airdrop failed");
    }
}
