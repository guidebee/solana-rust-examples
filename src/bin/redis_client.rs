use dotenv::dotenv;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

#[derive(serde::Deserialize)]
struct Env {
    wallet_pubkey: String,
}

async fn get_redis_client_with_retry() -> redis::RedisResult<Arc<Mutex<MultiplexedConnection>>> {
    let client = Client::open("redis://10.1.1.150:6379")?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(Arc::new(Mutex::new(conn)))
}

async fn get_balance(
    client: Arc<Mutex<MultiplexedConnection>>,
    wallet_address: &str,
) -> redis::RedisResult<String> {
    let balance_key = "balance";
    let mut conn = client.lock().await;
    let stored_balances_str: String = conn.hget(balance_key, wallet_address).await?;
    Ok(stored_balances_str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;
    let client = get_redis_client_with_retry().await?;

    let wallet_address = env.wallet_pubkey;

    match get_balance(client.clone(), &wallet_address[..]).await {
        Ok(balance) => {
            println!("Stored Balances String: {}", balance);
        }
        Err(err) => {
            eprintln!("Error fetching balance: {:?}", err);
        }
    }

    Ok(())
}
