use crate::models::BudgetEntry;
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub async fn get_entries_for_period(
    pool: &SqlitePool,
    pay_period_id: i64,
) -> sqlx::Result<Vec<BudgetEntry>> {
    sqlx::query_as("SELECT label, amount, entry_type FROM budget_entries WHERE pay_period_id = ?")
        .bind(pay_period_id)
        .fetch_all(pool)
        .await
}

pub async fn upsert_pay_period(
    pool: &SqlitePool,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<i64> {
    let id = sqlx::query_scalar(
        "INSERT OR IGNORE INTO pay_period (start_date, end_date) VALUES (?, ?)
         RETURNING id",
    )
    .bind(start)
    .bind(end)
    .fetch_optional(pool)
    .await?;

    // If IGNORE fired (row already existed), fall back to SELECT
    match id {
        Some(id) => Ok(id),
        None => {
            sqlx::query_scalar("SELECT id FROM pay_period WHERE start_date = ? AND end_date = ?")
                .bind(start)
                .bind(end)
                .fetch_one(pool)
                .await
        }
    }
}

pub async fn insert_budget_entry(
    pool: &SqlitePool,
    pay_period_id: i64,
    label: &str,
    amount: i64,
    entry_type: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO budget_entries (pay_period_id, label, amount, entry_type) VALUES (?, ?, ?, ?)",
    )
    .bind(pay_period_id)
    .bind(label)
    .bind(amount)
    .bind(entry_type)
    .execute(pool)
    .await?;
    Ok(())
}
