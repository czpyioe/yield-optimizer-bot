use sqlx::{PgPool, query};
use anyhow::Result;
use crate::db::models::ApySnapshot;


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


pub async fn get_recent_apys(pool: &PgPool) -> Result<Vec<ApySnapshot>> {
    let results = sqlx::query_as!(
        ApySnapshot,
        r#"
        SELECT protocol, network, asset, apy
        FROM apy_snapshots
        WHERE added_at > NOW() - INTERVAL '24 hour'
        ORDER BY apy DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
    Ok(results)
}