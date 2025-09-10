use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let program_address = Pubkey::from_str("11111111111111111111111111111111")?;
    let seeds: &[&[u8]] = &[b"helloWorld"];
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_address);

    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
    Ok(())
}