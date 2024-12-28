use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;
    let target: Pubkey = env.wallet_pubkey.parse()?;

    let client = RpcClient::new(env.rpc_url.to_string());

    // Replace with the wallet's public key
    let wallet_pubkey = target;

    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
        .expect("Invalid public key");
    // Get all token accounts by owner
    let token_accounts = client.get_token_accounts_by_owner(
        &wallet_pubkey,
        TokenAccountsFilter::ProgramId(token_program_id),
    )?;

    // Print the token accounts
    for token_account in token_accounts {
        println!("Token Account Pubkey: {}", token_account.pubkey);
        println!("Token Account Info: {:?}", token_account.account);
    }

    Ok(())
}
