use anyhow::Result;
use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;

use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_system_interface::instruction;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let private_key =
        env::var("PRIVATE_KEY_DEVELOPER").expect("PRIVATE_KEY_DEVELOPER not set in .env");
    let sender = Keypair::from_base58_string(&private_key);
    println!("Sender with public key: {}", &sender.pubkey());

    let recipient = Keypair::new();
    println!("recipient with public key: {}", &recipient.pubkey());

    let rpc_url = "http://10.1.1.160:8899";

    // Create a connection to Solana cluster
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let recipient = Keypair::new();

    // Check balance before transfer
    let pre_balance1 = client.get_balance(&sender.pubkey()).await?;
    let pre_balance2 = client.get_balance(&recipient.pubkey()).await?;

    // Instruction index for the System Program's transfer instruction
    let transfer_instruction_index: u32 = 2;

    // Define the amount to transfer
    let transfer_amount = LAMPORTS_PER_SOL / 100; // 0.01 SOL

    // Create instruction data manually (12 bytes: 4 for u32 index + 8 for u64 lamports)
    let mut instruction_data = Vec::with_capacity(12);
    instruction_data.extend_from_slice(&transfer_instruction_index.to_le_bytes());
    instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

    // Manually create the transfer instruction
    let transfer_instruction = Instruction {
        program_id: solana_system_interface::program::id(),
        accounts: vec![
            AccountMeta::new(sender.pubkey(), true), // from account, is signer and is writable
            AccountMeta::new(recipient.pubkey(), false), // to account, is not signer but is writable
        ],
        data: instruction_data,
    };

    println!("{:#?}", transfer_instruction);

    let transfer_instruction1 =
        instruction::transfer(&sender.pubkey(), &recipient.pubkey(), transfer_amount);

    println!("{:#?}", transfer_instruction1);

    // Fetch the latest blockhash and last valid block height
    let blockhash = client.get_latest_blockhash().await?;

    let mut transaction = Transaction::new_with_payer(
        &[transfer_instruction, transfer_instruction1],
        Some(&sender.pubkey()),
    );
    transaction.sign(&[&sender], blockhash);

    println!("{:#?}", transaction);

    let signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("{:#?}", signature);

    println!(
        "Sender prebalance: {}",
        pre_balance1 as f64 / LAMPORTS_PER_SOL as f64
    );
    println!(
        "Recipient prebalance: {}",
        pre_balance2 as f64 / LAMPORTS_PER_SOL as f64
    );

    // Check balance after transfer
    let post_balance1 = client.get_balance(&sender.pubkey()).await?;
    let post_balance2 = client.get_balance(&recipient.pubkey()).await?;

    println!(
        "Sender postbalance: {}",
        post_balance1 as f64 / LAMPORTS_PER_SOL as f64
    );
    println!(
        "Recipient postbalance: {}",
        post_balance2 as f64 / LAMPORTS_PER_SOL as f64
    );

    Ok(())
}
