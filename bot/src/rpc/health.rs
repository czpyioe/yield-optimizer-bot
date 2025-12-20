use alloy::providers::{Provider, ProviderBuilder};
use std::time::{Instant};
use anyhow::Result;
use futures::stream::{self,StreamExt};
use tokio::time::{timeout, Duration};
use crate::rpc::manager::{RpcEndpoint};

const CONCURRENT_CHECKS:usize = 50;
const RPC_TIMEOUT:Duration = Duration::from_secs(5);

pub async fn check_rpcs_health(endpoints:Vec<RpcEndpoint>)-> Result<Vec<RpcEndpoint>>{
    let results = stream::iter(endpoints)
        .map(|endpoint| async move {
            match timeout(RPC_TIMEOUT, check_one_rpc(endpoint.clone())).await {
                Ok(result)=>result,
                Err(_)=>{
                    let mut e = endpoint;
                    e.is_healthy = false;
                    e
                }
            }
        })
        .buffer_unordered(CONCURRENT_CHECKS)
        .collect()
        .await;    

    Ok(results)
}


async fn check_one_rpc(mut endpoint: RpcEndpoint)->RpcEndpoint{
    let start = Instant::now();
    let mut is_healthy = false;
    let mut latency = None; 

    match ProviderBuilder::new().connect(&endpoint.url).await{
        Ok(provider)=>{
            if provider.get_chain_id().await.is_ok() {
                is_healthy = true;
                latency = Some(start.elapsed());
            }
        },
        Err(_)=>{
            is_healthy=false
        }
    };

    endpoint.latency = latency;
    endpoint.is_healthy = is_healthy;
    endpoint.last_checked = Instant::now();

    endpoint
}