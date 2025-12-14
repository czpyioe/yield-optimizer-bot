use alloy::providers::{Provider, ProviderBuilder};
use dotenv::dotenv;
use std::{env, time};
use std::time::{Duration, Instant};


pub fn load_rpc_url() -> Vec<String>{
    dotenv().ok();
    let rpcs_urls: Vec<String> = env::vars().filter(|(i,_)| i.contains("RPC_URL")).map(|(_,v)| v).collect();
    rpcs_urls
}

pub async fn check_rpc_health()-> Vec<(String,f32,bool)>{
    let rpcs_urls = load_rpc_url();
    let mut rpcs_health : Vec<(String,f32,bool)> = vec![];
    for url in rpcs_urls{
        let start = Instant::now();
        let mut rpc_works = true;
        match ProviderBuilder::new().connect(&url).await{
            Ok(v)=>{
                if let Err(_) = v.get_block_number().await {
                    rpc_works = false;
                }
            },
            Err(_)=>{
                rpc_works=false
            }
        };
        let duration = start.elapsed();
        rpcs_health.push((url,duration.as_secs_f32(),rpc_works));
    }
    rpcs_health
}

