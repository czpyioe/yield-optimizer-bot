// use alloy::providers::{Provider, ProviderBuilder};
// use dotenv::dotenv;
// use std::{env, time};
// use std::time::{Duration, Instant};
// // use tokio;
// use anyhow::Result;
// use serde::Deserialize;


// // check health of the rpcs (keep only the best ones)
// pub async fn check_rpcs_health()-> Vec<(String,f32,bool)>{
//     let rpcs_urls = load_rpc_url();
//     let mut rpcs_health : Vec<(String,f32,bool)> = vec![];
//     for url in rpcs_urls{
//         rpcs_health.push(check_one_rpc(url).await);
//     }
//     rpcs_health
// }



// async fn check_one_rpc(url:String)->(String,f32,bool){
//     let start = Instant::now();
//     let mut rpc_works = true;
//     match ProviderBuilder::new().connect(&url).await{
//         Ok(v)=>{
//             if let Err(_) = v.get_chain_id().await {
//                 rpc_works = false;
//             }
//         },
//         Err(_)=>{
//             rpc_works=false
//         }
//     };
//     let duration = start.elapsed();
//     (url,duration.as_secs_f32(),rpc_works)
// }