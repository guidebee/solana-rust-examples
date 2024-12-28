use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
    keypair_secret: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env = envy::from_env::<Env>().unwrap();

    let rpc_url = env.rpc_url.to_string();
    let client = RpcClient::new(rpc_url.parse().unwrap());

    // Replace with the sender's and recipient's public keys
    let sender_keypair = Keypair::from_base58_string(&env.keypair_secret);
    let recipient_pubkey = Pubkey::from_str(&env.wallet_pubkey).expect("Invalid public key");

    // Amount to transfer (in lamports, 1 SOL = 1_000_000_000 lamports)
    let amount = 10_000; // 0.00001 SOL

    // Get the latest blockhash
    let latest_blockhash = client
        .get_latest_blockhash()
        .await
        .expect("Failed to get blockhash");

    // Create the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &sender_keypair.pubkey(),
            &recipient_pubkey,
            amount,
        )],
        Some(&sender_keypair.pubkey()),
        &[&sender_keypair],
        latest_blockhash,
    );

    // Send the transaction
    let signature = client
        .send_transaction_with_config(
            &transaction,
            RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: None,
                encoding: None,
                max_retries: Some(3),
                min_context_slot: None,
            },
        )
        .await
        .expect("Failed to send transaction");

    println!("Transaction signature: {}", signature);
}
