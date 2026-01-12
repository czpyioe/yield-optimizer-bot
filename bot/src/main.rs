mod rpc;
mod contracts;
mod db;
mod strategy;
mod telegram;

use anyhow::Result;
use teloxide::Bot;
use std::time::{Duration, Instant};
use std::env;

use crate::telegram::dispatcher;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ...");
    dotenv::dotenv().ok();
    
    // let mut pool = NetworkProviderPool::new(Duration::from_secs(300));
    // pool.initialize().await?;
    // let providers: std::collections::HashMap<Network, Vec<ProviderWithScore>> = pool.get_pools();

    let db_url = env::var("DATABASE_URL")?;
    let db_pool = db::pool::connect(&db_url).await?;

    let bot = Bot::from_env();

    let telegram_task = tokio::spawn(
        dispatcher::run(bot)
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

