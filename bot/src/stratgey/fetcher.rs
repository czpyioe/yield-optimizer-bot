use alloy::providers::Provider;
use alloy::primitives::Address;
use sqlx::PgPool;
use anyhow::Result;

use crate::contracts::addresses::Network;
use crate::contracts::protocols::aave::get_apy_snapshot;
use crate::db::queries::insert_apy_snapshot;

pub async fn fetch_and_store_aave_apy<P:Provider>(provider:P, network: Network,asset_address:Address,pool:&PgPool)->Result<f64>{
    let apy_snapshot = get_apy_snapshot(provider, network, asset_address).await?;

    insert_apy_snapshot(pool, apy_snapshot.clone()).await?;

    apy_snapshot.apy.ok_or_else(|| anyhow::anyhow!("APY not found"))
}