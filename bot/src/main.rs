mod rpc;
mod contracts;
mod db;
mod stratgey;
use anyhow::Result;
use std::time::Duration;
use std::{env};

use crate::stratgey::fetcher;
use crate::contracts::addresses::{Network,Asset};

#[tokio::main]
async fn main() -> Result<()> {

    println!("Starting...");

    let manager = rpc::manager::RpcManager::new(Duration::from_secs(10), Duration::from_secs(10));

    let provider = rpc::manager::RpcManager::init_test(manager).await?;
    

    let database_url = env::var("DATABASE_URL")?;

    println!("{database_url}");

    let pool = db::pool::connect(database_url.as_str()).await?;
    db::pool::run_migrations(&pool).await?;


    let network = Network::Ethereum;

    let asset_address = network.get_asset_address(Asset::USDC)?;

    println!("Network: {}, Asset_address: {}",network.name()?,asset_address);

    let apy_aave = fetcher::fetch_and_store_aave_apy(provider, Network::Ethereum, asset_address, &pool).await?;


    println!("APY of USDC: {}%", apy_aave);

    Ok(())
}