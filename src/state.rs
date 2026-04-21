use sqlx::SqlitePool;

#[derive(Clone)]
pub struct ApplicationState {
    pub pool: SqlitePool,
}

impl ApplicationState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(":memory:").await?;

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS pay_period (
                    id INTEGER PRIMARY KEY,
                    start_date TEXT NOT NULL,
                    end_date TEXT NOT NULL,
                    UNIQUE(start_date, end_date)
                )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS entries (
                    id INTEGER PRIMARY KEY,
                    pay_period_id INTEGER NOT NULL REFERENCES pay_period,
                    label TEXT NOT NULL,
                    amount INTEGER NOT NULL,
                    entry_type TEXT NOT NULL CHECK(entry_type IN ('income', 'expense'))
                )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}
