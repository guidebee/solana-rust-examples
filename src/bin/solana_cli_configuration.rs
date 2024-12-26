use solana_cli_config::Config;

#[tokio::main]
async fn main() {
    let config = Config::default();

    println!("JSON RPC URL: {}", config.json_rpc_url);
    let websocket_url = Config::compute_websocket_url(&config.json_rpc_url);
    println!("WebSock URL: {}", websocket_url);
    println!("Keypair path: {}", config.keypair_path);
    println!("Address labels: {:?}", config.address_labels);
    println!("Commitment: {}", config.commitment);
}
