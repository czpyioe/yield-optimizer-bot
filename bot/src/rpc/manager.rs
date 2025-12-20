use std::time::{Duration, Instant};
use anyhow::Result;

use crate::rpc::check_rpcs_health;
use crate::rpc::loader;
use crate::rpc::health;

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
    pub last_used:Instant
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
        let mut rpcs_endpoints:Vec<RpcEndpoint> = convert_stringvec_to_rpcendpointvec(rpcs_url)?;
        self.rpc_url.append(&mut rpcs_endpoints);
        self.last_rpc_fetch = Instant::now();
        self.refresh_health().await?;
        Ok(())
    }

    async fn refresh_health(&mut self) -> Result<()>{
        self.rpc_url = check_rpcs_health(self.rpc_url.clone()).await?;
        self.last_health_check = Instant::now();
        Ok(())
    }

    async fn fetch_new_rpcs(& mut self)-> Result<()>{
        let new_url = loader::load_rpcs_url().await?;
        let mut rpcs_endpoints:Vec<RpcEndpoint> = convert_stringvec_to_rpcendpointvec(new_url)?;
        self.rpc_url.append(&mut rpcs_endpoints);
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