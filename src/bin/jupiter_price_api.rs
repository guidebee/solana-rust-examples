#[macro_use] extern crate lazy_static;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref MINT_ID: String = String::from("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceInfo {
    id: String,
    #[serde(rename = "type")]
    price_type: String,
    price: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    data: HashMap<String, PriceInfo>,
    #[serde(rename = "timeTaken")]
    time_taken: f64,
}

use reqwest;


async fn fetch_price(token_id: &str) -> Result<ApiResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.jup.ag/price/v2?ids={}", token_id))
        .send()
        .await?;

    let price_response = response.json::<ApiResponse>().await?;
    Ok(price_response)
}

#[tokio::main]
async fn main() {
    match fetch_price(&MINT_ID[..]).await {
        Ok(response) => {
            println!("Price: {}", response.data[&MINT_ID[..]].price);
        }
        Err(e) => println!("Error: {}", e),
    }
}
