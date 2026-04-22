use sqlx::SqlitePool;
use crate::models::BudgetEntry;

pub async fn get_entries_for_period(pool: &SqlitePool, pay_period_id: i64) -> sqlx::Result<Vec<BudgetEntry>> {
    sqlx::query_as("SELECT label, amount, entry_type FROM budget_entries WHERE pay_period_id = ?")
        .bind(pay_period_id)
        .fetch_all(pool)
        .await
}