use alloy::providers::Provider;
use alloy::primitives::Address;
use sqlx::PgPool;
use anyhow::Result;

use crate::contracts::addresses::{Network,CompoundAsset};
use crate::contracts::protocols::aave;
use crate::contracts::protocols::compound;
use crate::db::queries::insert_apy_snapshot;

pub async fn fetch_and_store_aave_apy<P:Provider>(provider:P, network: Network,asset_address:Address,pool:&PgPool)->Result<f64>{
    let apy_snapshot = aave::get_apy_snapshot(provider, network, asset_address).await?;

    insert_apy_snapshot(pool, apy_snapshot.clone()).await?;

    apy_snapshot.apy.ok_or_else(|| anyhow::anyhow!("APY not found"))
}


pub async fn fetch_and_store_compound_apy<P:Provider>(provider:P, network: Network,compound_asset:CompoundAsset,pool:&PgPool)->Result<f64>{
    let apy_snapshot = compound::get_apy_snapshot(provider, network, compound_asset).await?;

    insert_apy_snapshot(pool, apy_snapshot.clone()).await?;

    apy_snapshot.apy.ok_or_else(|| anyhow::anyhow!("APY not found"))
}