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
    println!("Starting ...");
    dotenv::dotenv().ok();
    
    // let mut pool = NetworkProviderPool::new(Duration::from_secs(300));
    // pool.initialize().await?;
    // let providers: std::collections::HashMap<Network, Vec<ProviderWithScore>> = pool.get_pools();

    let db_url = env::var("DATABASE_URL")?;
    let db_pool = db::pool::connect(&db_url).await?;

    let telegram_token = env::var("TELEGRAM_BOT_API_KEY")?;
    let chat_id = env::var("CHAT_ID")?.parse::<i64>()?;
    let telegram_bot = telegram::client::TelegramBot::new(telegram_token, chat_id);
    
    let engine = strategy::logic::StrategyEngine::new(
        db_pool.clone(),
        telegram_bot.clone(),
    );

    let telegram_task = tokio::spawn(
        telegram::listener::run(telegram_bot.clone(), engine.clone())
    );

    println!("Bot running...");

    tokio::select! {
        res = telegram_task => {
            eprintln!("Telegram task stopped: {:?}", res);
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\nShutting down...");
        }
    }

    Ok(())
}

