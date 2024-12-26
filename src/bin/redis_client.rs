use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio;

async fn get_redis_client_with_retry() -> redis::RedisResult<Arc<Mutex<MultiplexedConnection>>> {
    let client  = Client::open("redis://10.1.1.150:6379")?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(Arc::new(Mutex::new(conn)))
}

async fn get_balance(client: Arc<Mutex<MultiplexedConnection>>, wallet_address: &str) -> redis::RedisResult<String> {
    let balance_key = "balance";
    let mut conn = client.lock().await;
    let stored_balances_str: String = conn.hget(balance_key, wallet_address).await?;
    Ok(stored_balances_str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_redis_client_with_retry().await?;

    let wallet_address = "your_wallet_address_in_base58";

    loop {
        match get_balance(client.clone(), wallet_address).await {
            Ok(balance) => {
                println!("Stored Balances String: {}", balance);
                break;
            }
            Err(err) => {
                // if err.is::<redis::RedisError>() && err.as_ref().unwrap().kind() == ErrorKind::ConnectionReset {
                //     eprintln!("Connection lost, retrying in 5 seconds: {:?}", err);
                //     tokio::time::sleep(Duration::from_secs(5)).await;
                // } else {
                //     eprintln!("Error fetching balance: {:?}", err);
                //     break;
                // }
                eprintln!("Error fetching balance: {:?}", err);
            }
        }
    }

    Ok(())
}
