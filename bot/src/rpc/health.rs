use alloy::providers::{Provider, ProviderBuilder};
use std::time::Instant;
use anyhow::Result;
use tokio::time::{timeout, Duration};
use crate::rpc::manager::RpcEndpoint;


pub async fn check_endpoint_health(mut endpoint: RpcEndpoint) -> Result<RpcEndpoint> {
    let result = timeout(Duration::from_secs(3), measure_latency(&endpoint.url)).await?;
    
    endpoint.latency = match result {
        Ok(latency) => Some(latency),
        _ => None,
    };
    Ok(endpoint)
}

async fn measure_latency(url: &str) -> Result<Duration> {
    let start = Instant::now();
    
    let provider = ProviderBuilder::new()
        .connect(url).await?;

    provider.get_chain_id().await?;
    
    Ok(start.elapsed())
}