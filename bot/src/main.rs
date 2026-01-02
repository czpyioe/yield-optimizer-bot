mod rpc;
mod contracts;
mod db;
mod strategy;

use anyhow::Result;
use futures::future::Inspect;
use std::time::{Duration, Instant};
use std::env;

use crate::rpc::manager::*;
use crate::contracts::addresses::Network;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ...");

    let mut pool = NetworkProviderPool::new(Duration::from_secs(300));

    pool.initialize().await?;

    let providers: std::collections::HashMap<Network, Vec<ProviderWithScore>> = pool.get_pools();

    let db_url = env::var("DATABASE_URL")?;
    let db_pool = db::pool::connect(&db_url).await?;

    strategy::orchestrator::snapshot_all_apys(providers,&db_pool).await?;

    Ok(())
}