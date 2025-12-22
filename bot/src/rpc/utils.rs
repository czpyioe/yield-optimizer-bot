use anyhow::Result;
use std::time::{Instant,Duration};

use crate::rpc::manager::{RpcEndpoint,RpcManager};


pub fn convert_stringvec_to_rpcendpointvec(string_vec:Vec<String>)->Result<Vec<RpcEndpoint>>{
    let rpcs_endpoints:Vec<RpcEndpoint> = string_vec.iter()
        .map(|url| RpcEndpoint{
        url:url.clone(),
        latency:None,
        is_healthy:false,
        last_checked:Instant::now(),
        use_count:0,
        last_used:Instant::now()
    }).collect();
    Ok(rpcs_endpoints)
}


pub fn compute_rpc_score(rpc_endpoint:&RpcEndpoint)->Option<usize>{
    let mut score = rpc_endpoint.latency?.as_millis() as usize;

    match rpc_endpoint.last_used.elapsed().as_secs_f64(){
        n if n<10.0=>score+=200,
        n if n<30.0=>score+=100,
        n if n<60.0=>score+=50,
        _=> ()
    }
    Some(score)
}


pub fn stats(rpc_manager:&RpcManager) -> (usize, usize, Option<usize>) {
    let total = rpc_manager.rpc_url.len();
    let healthy = rpc_manager.rpc_url.iter().filter(|e| e.is_healthy).count();
    
    let (sum, count) = rpc_manager.rpc_url
        .iter()
        .filter(|e| e.is_healthy)
        .filter_map(|e| e.latency)
        .fold((Duration::ZERO, 0u64), |(acc, n), latency| {
            (acc + latency, n + 1)
        });

    let avg_latency = if count > 0 {
        Some((sum.as_millis() / count as u128) as usize)
    } else {
        None
    };

    (healthy, total, avg_latency)
}