use alloy::network;
use dotenv::dotenv;
use std::{env};
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

use crate::contracts::addresses::Network;
use crate::rpc::utils;

#[derive(Debug, Deserialize,Clone)]
pub struct Chain {
    chainId:usize,
    rpc: Vec<RpcUrl>,
}


#[derive(Debug,Deserialize,Clone)]
pub struct RpcUrl{
    url:String
}


// should load .env rpc and lama rpc form the api
pub async fn load_rpcs_url() -> Result<HashMap<Network,Vec<String>>>{
    let mut all_rpcs:HashMap<Network,Vec<String>> = HashMap::new();
    if let Ok(rpcs) = load_env_rpcs(){
        for (network, urls) in rpcs{
            all_rpcs.entry(network).or_insert_with(Vec::new).extend(urls);
        }
    }
    if let Ok(rpcs) = request_lama_rpcs().await{
        for (network, urls) in rpcs{
            all_rpcs.entry(network).or_insert_with(Vec::new).extend(urls);
        }
    }
    Ok(all_rpcs)
}


fn load_env_rpcs() -> Result<HashMap<Network,Vec<String>>>{
    dotenv().ok();
    let mut rpcs_urls: HashMap<Network,Vec<String>> = HashMap::new();
    for (k,v) in env::vars(){
        if k.starts_with("RPC_URL"){
            let network = if k.starts_with("ETH") {
                Network::Ethereum
            } else if k.starts_with("ARB") {
                Network::Arbitrum
            } else {
                continue;
            };
            rpcs_urls.entry(network).or_insert_with(Vec::new).push(v);
        }
    }
    Ok(rpcs_urls)
}

pub async fn request_lama_rpcs()->Result<HashMap<Network,Vec<String>>>{
    let mut rpcs_urls: HashMap<Network,Vec<String>> = HashMap::new();
    let url: &str = "https://chainlist.org/rpcs.json";
    let body:Vec<Chain> = reqwest::get(url)
        .await?
        .json()
        .await?;

    for chain in body{
        let network = Network::get_network_from_chain_id(chain.chainId)?;

        let urls:Vec<String> = chain.rpc.into_iter()
            .map(|rpc| rpc.url)
            .filter(|url| utils::is_valid_http_rpc_url(url))
            .collect();

        rpcs_urls.entry(network).or_insert_with(Vec::new).extend(urls);
    }
    
    Ok(rpcs_urls)
}
