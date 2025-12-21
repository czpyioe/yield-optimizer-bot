use std::time::{Duration, Instant};
use anyhow::Result;
use std::collections::HashSet;

use crate::rpc::loader;
use crate::rpc::health;

pub struct RpcManager{
    pub rpc_url: Vec<RpcEndpoint>,
    current_index: usize,
    last_health_check: Instant,
    last_rpc_fetch: Instant,
    health_check_interval: Duration,
    rpc_fetch_interval: Duration
}

#[derive(Clone)]
pub struct RpcEndpoint{
    pub url: String,
    pub latency: Option<Duration>,
    pub is_healthy: bool,
    pub last_checked: Instant,
    pub use_count:u32,
    pub last_used:Instant
}


impl RpcManager{
    pub fn new(health_check_interval:Duration,rpc_fetch_interval:Duration) -> Self {
        Self {
            rpc_url: Vec::new(),
            current_index:0,
            last_health_check: Instant::now(),
            last_rpc_fetch: Instant::now(),
            health_check_interval,
            rpc_fetch_interval,
        }
    }

    async fn initialize(&mut self)-> Result<()>{
        let rpcs_url = loader::load_rpcs_url().await?;
        self.rpc_url = convert_stringvec_to_rpcendpointvec(rpcs_url)?;
        self.last_rpc_fetch = Instant::now();
        self.refresh_health().await?;
        Ok(())
    }

    async fn get_provider(&mut self)->Result<()>{
        Ok(())
    }

    async fn refresh_health(&mut self) -> Result<()>{
        self.rpc_url = health::check_rpcs_health(self.rpc_url.clone()).await?;
        self.last_health_check = Instant::now();
        Ok(())
    }

    async fn fetch_new_rpcs(& mut self)-> Result<()>{
        let existing_urls:HashSet<&str> = self.rpc_url
            .iter()
            .map(|i| i.url.as_str())
            .collect();

        let fetched_urls = loader::load_rpcs_url().await?;
        let fetched_endpoints = convert_stringvec_to_rpcendpointvec(fetched_urls)?;

        let mut new_url: Vec<RpcEndpoint> = fetched_endpoints
            .into_iter()
            .filter(|i| !existing_urls.contains(i.url.as_str()))
            .collect();

        self.rpc_url.append(&mut new_url);
        self.last_health_check = Instant::now();
        self.refresh_health().await?;
        Ok(())
    }
}


fn convert_stringvec_to_rpcendpointvec(string_vec:Vec<String>)->Result<Vec<RpcEndpoint>>{
    let rpcs_endpoints:Vec<RpcEndpoint> = string_vec.iter()
        .map(|url| RpcEndpoint{
        url:url.clone(),
        latency:None,
        is_healthy:true,
        last_checked:Instant::now(),
        use_count:0,
        last_used:Instant::now()
    }).collect();
    Ok(rpcs_endpoints)
}