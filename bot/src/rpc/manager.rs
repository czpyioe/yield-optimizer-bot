use std::time::{Duration, Instant};
use anyhow::Result;

use crate::rpc::loader;
use crate::rpc::health;

pub struct RpcManager{
    rpc_url: Vec<RpcEndpoint>,
    last_health_check: Instant,
    last_rpc_fetch: Instant,
    health_check_interval: Duration,
    rpc_fetch_interval: Duration
}

pub struct RpcEndpoint{
    url: String,
    latency: Option<Duration>,
    is_healthy: bool,
    last_checked: Instant,
    use_count:u32,
    last_used:Instant
}


impl RpcManager{
    pub fn new(health_check_interval:Duration,rpc_fetch_interval:Duration) -> Self {
        Self { 
            rpc_url: Vec::new(), 
            last_health_check: Instant::now(), 
            last_rpc_fetch: Instant::now(), 
            health_check_interval,
            rpc_fetch_interval, 
        }
    }

    async fn get_provider(&mut self)-> Result<()>{
        let rpcs_url = loader::load_rpcs_url().await?;
        let mut rpcs_endpoints:Vec<RpcEndpoint> = rpcs_url.iter()
            .map(|url| RpcEndpoint{
            url:url.clone(),
            latency:None,
            is_healthy:true,
            last_checked:Instant::now(),
            use_count:0,
            last_used:Instant::now()
        }).collect();
        self.rpc_url.append(&mut rpcs_endpoints);
        self.last_rpc_fetch = Instant::now();
        Ok(())
    }

    async fn refresh_health() -> Result<()>{
        Ok(())
    }

    async fn fetch_new_rpcs(& self)-> Result<()>{
        let new_url = loader::load_rpcs_url().await?;
        Ok(())
    }
}