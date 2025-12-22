use anyhow::Result;
use std::time::Instant;

use crate::rpc::manager::RpcEndpoint;


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