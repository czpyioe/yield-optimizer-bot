use anyhow::Result;
use alloy::{dyn_abi::parser::Error, network, providers::{self, Provider}};
use sqlx::PgPool;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{contracts::{addresses::{Asset, Network,Protocol}, protocols}, strategy::fetcher};
use crate::rpc::manager::ProviderWithScore;

#[derive(Debug, Clone, Copy)]
struct SnapshotTask{
    protocol:Protocol,
    network:Network,
    asset:Asset
}

#[derive(Clone)]
struct DynamicProvider{
    provider: Arc<dyn Provider>,
    endpoint_url:String,
    dynamic_score:usize
}

impl DynamicProvider{
    fn from_provider_with_score(p:&ProviderWithScore)->Self{
        Self { 
            provider: p.provider.clone(),
            endpoint_url:p.endpoint.url.clone(),
            dynamic_score: p.score 
        }
    }

    fn add_usage_penalty(&mut self) {
        self.dynamic_score += 50;
    }

    fn penalize_failure(&mut self) {
        self.dynamic_score+=500
    }
}


#[derive(Clone)]
struct DynamicProviderPool{
    providers:Vec<DynamicProvider>
}

impl DynamicProviderPool{
    fn new(providers:Vec<ProviderWithScore>)->Self{
        Self { 
            providers: providers.iter().map(DynamicProvider::from_provider_with_score).collect()
        }
    }

    fn get_best(&mut self) -> Option<DynamicProvider> {
        if self.providers.is_empty(){
            return None;
        }
        let (best_ind,_) = self.providers
            .iter()
            .enumerate()
            .min_by_key(|(_,p)| p.dynamic_score)?;

        let best = self.providers[best_ind].clone();
        self.providers[best_ind].add_usage_penalty();

        Some(best)
    }

    fn report_failure(&mut self, endpoint_url: &str) {
        if let Some(provider) = self.providers.iter_mut().find(|p| p.endpoint_url == endpoint_url) {
            provider.penalize_failure();
        }
    }


}



pub async fn snapshot_all_apys(pools:HashMap<Network, Vec<ProviderWithScore>>, pool:&PgPool) ->Result<()>{
    let dynamic_pools:HashMap<Network,Arc<Mutex<DynamicProviderPool>>> = pools
        .into_iter()
        .map(|(network,providers)| {
            (network,Arc::new(Mutex::new(DynamicProviderPool::new(providers))))
        })
        .collect();

    let tasks = get_all_tasks();

    let results: Vec<Result<(SnapshotTask, f64), (SnapshotTask, anyhow::Error)>> = stream::iter(tasks)
        .map(|task| {
            let db_pool = pool.clone();
            let provider_pool = dynamic_pools.get(&task.network).cloned();

            async move {
                match provider_pool {
                    Some(dynamic_pool) => {
                        match fetch_with_retry(dynamic_pool, task, &db_pool).await {
                            Ok(apy) => Ok((task, apy)),
                            Err(e) => Err((task, e)),
                        }
                    },
                    None => Err((
                        task,
                        anyhow::anyhow!("No providers for network {:?}", task.network),
                    )),
                }
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    Ok(())
}


async fn fetch_with_retry(
    dynamic_pool: Arc<Mutex<DynamicProviderPool>>,
    task: SnapshotTask,
    db_pool: &PgPool
) -> Result<f64> {
    const MAX_RETRIES: usize = 3;
    
    for attempt in 0..MAX_RETRIES {
        let (provider, endpoint_url) = {
            let mut pool = dynamic_pool.lock().await;
            match pool.get_best() {
                Some(p) => (p.provider.clone(), p.endpoint_url.clone()),
                None => return Err(anyhow::anyhow!("No available providers")),
            }
        };

        match fetcher::fetch_and_store_apy(
            provider,
            task.protocol,
            task.network,
            task.asset,
            db_pool
        ).await {
            Ok(apy) => return Ok(apy),
            Err(e) => {
                {
                    let mut pool = dynamic_pool.lock().await;
                    pool.report_failure(&endpoint_url);
                }
                
                if attempt >= MAX_RETRIES - 1 {
                    return Err(anyhow::anyhow!(
                        "Failed after {} attempts. Last error: {}",
                        MAX_RETRIES,
                        e
                    ));
                }
            }
        }
    }
    Ok(())
}


fn get_all_tasks()->Vec<SnapshotTask> {
    let mut tasks:Vec<SnapshotTask> = Vec::new();
    for protocol in Protocol::all(){
        for network in protocol.supported_networks(){
            for asset in protocol.supported_assets(&network){
                tasks.push(SnapshotTask {
                    protocol,
                    network,
                    asset,
                });
            }
        }
    }
    tasks
}