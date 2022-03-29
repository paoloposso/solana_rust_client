use solana_client::rpc_client::RpcClient;
use solana_rust_client::{check_balance, request_air_drop, transfer_funds, create_keypair};
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    let amount = 0.5;

    if let Ok(airdrop_signature) = request_air_drop(&rpc, &sender.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        match transfer_funds(&rpc, &sender, &receiver.pubkey(), amount) {
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
