use sqlx::{PgPool, query};
use anyhow::Result;
use crate::db::models::{Position,ApySnapshot};


pub async fn insert_position(pool:&PgPool,position:Position)->Result<()>{
    query(
        "INSERT INTO positions (protocol, network, asset, amount, apy) VALUES ($1, $2, $3, $4, $5)")
        .bind(position.protocol)
        .bind(position.network)
        .bind(position.asset_address)
        .bind(position.amount)
        .bind(position.apy)
        .execute(pool)
        .await?;
    Ok(())
}


pub async fn insert_apy_snapshot(pool:&PgPool,apy_snapshot:ApySnapshot)->Result<()>{
    query(
        "INSERT INTO apy_snapshots (protocol, network, asset, apy) VALUES ($1, $2, $3, $4)")
        .bind(apy_snapshot.protocol)
        .bind(apy_snapshot.network)
        .bind(apy_snapshot.asset)
        .bind(apy_snapshot.apy)
        .execute(pool)
        .await?;
    Ok(())
}