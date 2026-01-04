mod rpc;
mod contracts;
mod db;
mod strategy;
mod telegram;

use anyhow::Result;
use std::time::{Duration, Instant};
use std::env;

use crate::rpc::manager::*;
use crate::contracts::addresses::Network;
use crate::telegram::client;
use crate::strategy::logic;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    println!("Starting ...");

    let mut pool = NetworkProviderPool::new(Duration::from_secs(300));
    pool.initialize().await?;
    let providers: std::collections::HashMap<Network, Vec<ProviderWithScore>> = pool.get_pools();

    let db_url = env::var("DATABASE_URL")?;
    let db_pool = db::pool::connect(&db_url).await?;
    println!("db url: {}",db_url);
    // db::pool::run_migrations(&db_pool).await;

    db::orchestrator::snapshot_all_apys(providers,&db_pool).await?;

    let telegram_api_token = env::var("TELEGRAM_BOT_API_KEY")?;
    let chat_id = env::var("CHAT_ID")?.parse::<i64>()?;
    let bot = client::TelegramBot::new(telegram_api_token);

    let stratgey_engine = logic::StartegyEngine::new(db_pool.clone(),bot.clone(),chat_id);

    println!("strategy engine loaded");

    match stratgey_engine.analyze_and_notify().await{
        Ok(_)=>println!("working"),
        Err(e)=>println!("error: {}",e)
    }

    Ok(())
}

