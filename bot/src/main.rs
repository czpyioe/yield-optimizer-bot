mod rpc;
mod contracts;
mod db;
use alloy::providers::{Provider, ProviderBuilder}; 
use anyhow::Result;
use std::time::Duration;

use dotenv::dotenv;
use std::{env};

enum network_choice{
    Ethereum,
    Arbitrum,
    Base,
}

#[tokio::main]
async fn main() -> Result<()> {

    println!("Starting...");

    // let manager = rpc::RpcManager::new(Duration::from_secs(10), Duration::from_secs(10));

    // let provider = rpc::RpcManager::init_test(manager).await?;
    
    // contracts::protocols::aave::get_aave_tokens_reserves(provider,network_choice::Ethereum).await?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = db::pool::connect(database_url.as_str()).await?;
    db::pool::run_migrations(&pool).await?;

    Ok(())
}