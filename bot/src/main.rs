// bot/src/main.rs
mod rpc;
mod contracts;
mod db;
mod stratgey;

use anyhow::Result;
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ...");

    let manager = rpc::manager::RpcManager::new(
        Duration::from_secs(600),
        Duration::from_secs(3600)
    );
    let provider = rpc::manager::RpcManager::init_test(manager).await?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = db::pool::connect(&database_url).await?;
    db::pool::run_migrations(&pool).await?;

    // 3. Run snapshot avec streaming parallèle
    stratgey::orchestrator::snapshot_all_apys(provider, &pool).await?;

    Ok(())
}