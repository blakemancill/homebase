use anyhow::Context;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ApplicationState {
    pub pool: SqlitePool,
}

impl ApplicationState {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let pool = SqlitePool::connect(&db_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
}
