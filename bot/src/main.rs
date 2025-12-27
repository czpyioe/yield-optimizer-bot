mod rpc;
mod contracts;
mod db;
mod stratgey;
use anyhow::Result;
use std::time::Duration;
use std::{env};

use crate::stratgey::fetcher;
use crate::contracts::addresses::{CompoundAsset, CompoundContract, Network};

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
    let compound_asset = CompoundAsset::USDC;

    let contract_address = network.get_compound_contract_address(CompoundContract::cTokenv3,compound_asset)?;

    println!("Network: {}, Contract address: {}",network.name()?,contract_address);

    let apy_aave = fetcher::fetch_and_store_compound_apy(provider, Network::Ethereum, compound_asset, &pool).await?;


    println!("APY of USDC: {}%", apy_aave);

    Ok(())
}