use anyhow::Result;
use dotenv::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::{
    id as token_2022_program_id, instruction::initialize_mint, state::Mint,
};

use std::env;

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let private_key =
        env::var("PRIVATE_KEY_DEVELOPER").expect("PRIVATE_KEY_DEVELOPER not set in .env");
    let fee_payer = Keypair::from_base58_string(&private_key);
    println!(
        "Generated new keypair with public key: {}",
        &fee_payer.pubkey()
    );

    let rpc_url = "http://10.1.1.160:8899";

    // Create a connection to Solana cluster
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Generate keypair to use as address of mint
    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    // Create account instruction
    let create_account_instruction = create_account(
        &fee_payer.pubkey(),      // fee payer
        &mint.pubkey(),           // mint address
        rent,                     // rent
        space as u64,             // space
        &token_2022_program_id(), // program id
    );

    // Initialize mint instruction
    let initialize_mint_instruction = initialize_mint(
        &token_2022_program_id(),
        &mint.pubkey() as &Pubkey, // mint address
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        9,                         // decimals
    )?;

    let recent_blockhash = client.get_latest_blockhash().await?;
    // Create transaction and add instructions
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash,
    );

    // Send and confirm transaction
    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("Mint Address: {}", mint.pubkey());
    println!("Transaction Signature: {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    println!("{:#?}", account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    println!("{:#?}", mint_account);

    Ok(())
}
