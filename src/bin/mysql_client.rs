use dotenv::dotenv;
use mysql_async::prelude::*;
use mysql_async::{OptsBuilder, Pool, Row};
use serde::Deserialize;
use tokio;

#[derive(serde::Deserialize)]
struct Env {
    mysql_host: String,
    mysql_user: String,
    mysql_password: String,
    mysql_database: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct JupiterAmountHistory {
    timestamp: String,
    sol_amount: f64,
    wrapped_sol_amount: f64,
    usdc_amount: f64,
    sol_unit_price: f64,
    normalized_sol_amount: f64,
    normalized_usdc_amount: f64,
}

async fn get_last_10_jupiter_trade_history_entries(
    pool: &Pool,
    table_name: &str,
) -> Result<Vec<JupiterAmountHistory>, mysql_async::Error> {
    let query_string = format!(
        "SELECT bar_time as timestamp, sol_amount as solAmount, wrapped_sol_amount as wrappedSolAmount, \
        usdc_amount as usdcAmount, sol_unit_price as solUnitPrice, normalized_sol_amount as normalizedSolAmount, \
        normalized_usdc_amount as normalizedUSDCAmount FROM {} ORDER BY bar_time DESC LIMIT 10",
        table_name
    );

    let mut conn = pool.get_conn().await?;
    let result: Vec<Row> = conn.query(query_string).await?;

    let history: Vec<JupiterAmountHistory> = result
        .into_iter()
        .map(|row| {
            let (
                timestamp,
                sol_amount,
                wrapped_sol_amount,
                usdc_amount,
                sol_unit_price,
                normalized_sol_amount,
                normalized_usdc_amount,
            ) = mysql_async::from_row(row);

            JupiterAmountHistory {
                timestamp,
                sol_amount,
                wrapped_sol_amount,
                usdc_amount,
                sol_unit_price,
                normalized_sol_amount,
                normalized_usdc_amount,
            }
        })
        .collect();

    Ok(history)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env = envy::from_env::<Env>()?;

    let opts = OptsBuilder::default()
        .user(Some(env.mysql_user))
        .pass(Some(env.mysql_password))
        .db_name(Some(env.mysql_database))
        .ip_or_hostname(env.mysql_host);
    let pool = Pool::new(opts);

    match get_last_10_jupiter_trade_history_entries(&pool, "jupiter_jupitertradehistory").await {
        Ok(history) => {
            println!("Trade History: {:?}", history);
        }
        Err(err) => {
            eprintln!("Error fetching trade history: {:?}", err);
        }
    }

    pool.disconnect().await?;

    Ok(())
}
