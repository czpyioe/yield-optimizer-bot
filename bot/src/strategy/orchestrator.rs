use anyhow::Result;
use alloy::{providers::Provider};
use sqlx::PgPool;
use futures::stream::{self, StreamExt};

use crate::{contracts::{addresses::{Asset, Network,Protocol}, protocols}, strategy::fetcher};

#[derive(Debug, Clone, Copy)]
struct SnapshotTask{
    protocol:Protocol,
    network:Network,
    asset:Asset
}

pub async fn snapshot_all_apys<P:Provider + Clone>(provider:P, pool:&PgPool) ->Result<()>{
    let tasks = get_all_tasks();

    let results:Vec<_>= stream::iter(tasks)
        .map(|task| {
            let provider = provider.clone();
            let pool = pool.clone();
            async move{
                match fetcher::fetch_and_store_apy(
                    provider, 
                    task.protocol, 
                    task.network, 
                    task.asset, 
                    &pool
                ).await {
                    Ok(apy) => Ok((task,apy)),
                    Err(e)=>Err((task,e))
                }
            }
})
        .buffer_unordered(50)
        .collect()
        .await;
    Ok(())
}


fn get_all_tasks()->Vec<SnapshotTask> {
    let mut tasks:Vec<SnapshotTask> = Vec::new();
    for protocol in Protocol::all(){
        for network in protocol.supported_networks(){
            for asset in protocol.supported_assets(){
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