use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ApplicationState {
    pub pool: SqlitePool,
}

impl ApplicationState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect("sqlite://finance_tracker.db?mode=rwc").await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
}
