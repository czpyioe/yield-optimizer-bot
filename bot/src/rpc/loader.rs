// use alloy::providers::{Provider, ProviderBuilder};
// use dotenv::dotenv;
// use std::{env, time};
// use std::time::{Duration, Instant};
// // use tokio;
// use anyhow::Result;
// use serde::Deserialize;


// #[derive(Debug, Deserialize)]
// pub struct Chain {
//     chain:String,
//     rpc: Vec<RpcUrl>,
// }


// #[derive(Debug,Deserialize)]
// pub struct RpcUrl{
//     url:String
// }

// // should load .env rpc and lama rpc form the api
// async fn load_rpcs_url() -> Result<Vec<String>>{
//     let mut all_rpcs:Vec<String> = Vec::new();
//     if let Ok(mut v) = load_env_rpcs(){
//         all_rpcs.append(&mut v);
//     }
//     if let Ok(mut v) = request_lama_rpcs().await{
//         all_rpcs.append(&mut v);
//     }
//     Ok(all_rpcs)
// }


// pub fn load_env_rpcs() -> Result<Vec<String>>{
//     dotenv().ok();
//     let rpcs_urls: Vec<String> = env::vars().filter(|(i,_)| i.contains("RPC_URL")).map(|(_,v)| v).collect();
//     Ok(rpcs_urls)
// }

// pub async fn request_lama_rpcs()->Result<Vec<String>>{
//     let url = "https://chainlist.org/rpcs.json";
//     let body:Vec<Chain> = reqwest::get(url)
//         .await?
//         .json()
//         .await?;

//     let eth_urls = body.into_iter().filter(|i| i.chain=="ETH").flat_map(|c| c.rpc).map(|rpc| rpc.url).collect();
//     Ok(eth_urls)
// }
