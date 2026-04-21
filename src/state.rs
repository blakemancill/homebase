use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ApplicationState {
    pub pool: SqlitePool,
}

impl ApplicationState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(":memory:").await?;
        Ok(Self { pool })
    }
}