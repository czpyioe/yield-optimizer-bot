use sqlx::{PgPool, query};
use anyhow::Result;
use crate::db::models::{Position};


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