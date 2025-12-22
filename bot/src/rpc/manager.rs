use std::time::{Duration, Instant};
use anyhow::Result;
use std::collections::HashSet;

use crate::rpc::loader;
use crate::rpc::health;
use crate::rpc::utils;

pub struct RpcManager{
    pub rpc_url: Vec<RpcEndpoint>,
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
    pub last_used:Instant,
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

    pub async fn initialize(&mut self)-> Result<()>{
        let rpcs_url = loader::load_rpcs_url().await?;
        self.rpc_url = utils::convert_stringvec_to_rpcendpointvec(rpcs_url)?;
        self.last_rpc_fetch = Instant::now();
        self.refresh_health().await?;
        Ok(())
    }

    // get provider/rotate
    pub async fn get_rpc_url(&mut self) -> Result<String> {
        let best_rpc = self.select_best_rpc()?;
        best_rpc.use_count += 1;
        best_rpc.last_used = Instant::now();

        Ok(best_rpc.url.clone())
    }

    fn select_best_rpc(&mut self) -> Result<&mut RpcEndpoint> {
        let best = self.rpc_url
            .iter()
            .enumerate()
            .filter(|(_, rpc)| rpc.is_healthy && rpc.latency.is_some())
            .filter_map(|(idx, rpc)| {
                utils::compute_rpc_score(rpc).map(|score| (idx, score))
            })
            .min_by_key(|&(_, score)| score);

        let (best_idx, _) = best.ok_or_else(|| anyhow::anyhow!("no healthy RPC available"))?;

        Ok(&mut self.rpc_url[best_idx])
    }

    async fn refresh_health(&mut self) -> Result<()>{
        self.rpc_url = health::check_rpcs_health(self.rpc_url.clone()).await?;
        self.last_health_check = Instant::now();
        Ok(())
    }

    async fn fetch_new_rpcs(&mut self)-> Result<()>{
        let existing_urls:HashSet<&str> = self.rpc_url
            .iter()
            .map(|i| i.url.as_str())
            .collect();

        let fetched_urls = loader::load_rpcs_url().await?;
        let fetched_endpoints = utils::convert_stringvec_to_rpcendpointvec(fetched_urls)?;

        let mut new_url: Vec<RpcEndpoint> = fetched_endpoints
            .into_iter()
            .filter(|i| !existing_urls.contains(i.url.as_str()))
            .collect();

        self.rpc_url.append(&mut new_url);
        self.last_rpc_fetch = Instant::now();
        self.refresh_health().await?;
        Ok(())
    }
}

