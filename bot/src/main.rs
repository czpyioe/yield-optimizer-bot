mod rpc;
mod contracts;
use alloy::providers::{Provider, ProviderBuilder}; 
use anyhow::Result;

enum network_choice{
    Ethereum,
    Arbitrum,
    Base,
}

#[tokio::main]
async fn main() -> Result<()> {

    let provider = ProviderBuilder::new().connect_http("https://mainnet.infura.io/v3/0635adb2d8d644188490eb2cfe091818".parse()?); 
    
    contracts::protocols::aave::get_aave_tokens_reserves(provider,network_choice::Ethereum).await?;
    
    Ok(())
}