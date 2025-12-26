use sqlx::PgPool;
use anyhow::Result;

pub async fn connect(db_url:&str)->Result<PgPool>{
    Ok(PgPool::connect(db_url).await?)
}

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}