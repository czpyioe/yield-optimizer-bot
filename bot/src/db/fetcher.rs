use alloy::providers::{Provider, ProviderBuilder};
use sqlx::PgPool;
use anyhow::Result;

use crate::contracts::addresses::{Network,Asset,Protocol};
use crate::contracts::protocols::{compound,aave};
use crate::db::queries::insert_apy_snapshot;


pub async fn fetch_and_store_apy(endpoint_url:String, protocol: Protocol, network: Network,asset:Asset,pool:&PgPool)->Result<f64>{
    let provider = ProviderBuilder::new().connect(endpoint_url.as_str()).await?;
    let apy_snapshot = match protocol{
        Protocol::Compound=>compound::get_apy_snapshot(provider, network, asset).await?,
        Protocol::Aave=>aave::get_apy_snapshot(provider, network, asset).await?
    };

    insert_apy_snapshot(pool, apy_snapshot.clone()).await?;

    apy_snapshot.apy.ok_or_else(|| anyhow::anyhow!("APY not found"))
}