use crate::common::TestApp;

impl TestApp {
    pub async fn create_pay_period(&self, user_id: i64, start_date: &str, end_date: &str) -> i64 {
        sqlx::query_scalar!(
            "INSERT INTO pay_period (user_id, start_date, end_date) VALUES (?, ?, ?) RETURNING id",
            user_id,
            start_date,
            end_date,
        )
            .fetch_one(&self.pool)
            .await
            .expect("failed to create pay period")
            .expect("INSERT ... RETURNING id should yield a row")
    }
}