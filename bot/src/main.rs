// bot/src/main.rs
mod rpc;
mod contracts;
mod db;
mod strategy;

use anyhow::Result;
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ...");

    rpc::loader::request_lama_rpcs().await?;

    Ok(())
}