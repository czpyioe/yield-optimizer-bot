use std::time::{Duration, Instant};

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
    pub fn new(health_check_interval:Duration,rpc_fetch_interval:Duration) -> Self{
        Self { 
            rpc_url: Vec::new(), 
            last_health_check: Instant::now(), 
            last_rpc_fetch: Instant::now(), 
            health_check_interval,
            rpc_fetch_interval, 
        }
    }
}