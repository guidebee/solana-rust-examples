use dotenv::dotenv;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
    keypair_secret: String,
    mint_account_pubkey: String,
}

const MAX_RETRIES: usize = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let mint_id: Pubkey = env.mint_account_pubkey.parse()?;
    let sender_keypair = Keypair::from_base58_string(&env.keypair_secret);
    let recipient_pubkey = Pubkey::from_str(&env.wallet_pubkey)?;
    let client = RpcClient::new(env.rpc_url.to_string());

    // JUP token mint address
    let token_mint = mint_id;

    let sender_token_account = get_associated_token_address(&sender_keypair.pubkey(), &token_mint);
    let recipient_token_account = get_associated_token_address(&recipient_pubkey, &token_mint);

    // Amount to transfer (in token's smallest unit, e.g., 1 token = 10^6 smallest units)
    let amount = 1_000; // Adjust based on token's decimals, e.g., 0.00001 JUP

    let mut retries = 0;
    let mut success = false;
    // Get the latest blockhash
    let mut latest_blockhash = client.get_latest_blockhash()?;

    while retries < MAX_RETRIES && !success {
        // Create the transaction
        // Create the transfer instruction
        let transfer_instruction = token_instruction::transfer(
            &spl_token::id(),
            &sender_token_account,
            &recipient_token_account,
            &sender_keypair.pubkey(),
            &[],
            amount,
        )?;
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&sender_keypair.pubkey()),
            &[&sender_keypair],
            latest_blockhash,
        );

        // Send the transaction
        let signature = client.send_transaction_with_config(
            &transaction,
            RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: None,
                encoding: None,
                max_retries: None,
                min_context_slot: None,
            },
        );
        match signature {
            Ok(sig) => {
                println!("Transaction signature: {}", sig);
                success = true;
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                retries += 1;
                latest_blockhash = client.get_latest_blockhash()?;
            }
        }
    }

    Ok(())
}
