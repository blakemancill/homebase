use crate::models::{BudgetEntry, EntryType};
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub async fn get_entries_for_period(
    pool: &SqlitePool,
    pay_period_id: i64,
) -> sqlx::Result<Vec<BudgetEntry>> {
    sqlx::query_as!(
        BudgetEntry,
        r#"SELECT id, label, amount, entry_type as "entry_type: EntryType"
           FROM budget_entries WHERE pay_period_id = ?"#,
        pay_period_id
    )
    .fetch_all(pool)
    .await
}

pub async fn upsert_pay_period(
    pool: &SqlitePool,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<i64> {
    sqlx::query!(
        "INSERT OR IGNORE INTO pay_period (start_date, end_date) VALUES (?, ?)",
        start,
        end
    )
    .execute(pool)
    .await?;

    sqlx::query_scalar!(
        "SELECT id FROM pay_period WHERE start_date = ? AND end_date = ?",
        start,
        end
    )
    .fetch_one(pool)
    .await
    .map(|id: Option<i64>| id.expect("id is always set"))
}

pub async fn insert_budget_entry(
    pool: &SqlitePool,
    pay_period_id: i64,
    label: &str,
    amount: i64,
    entry_type: EntryType,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO budget_entries (pay_period_id, label, amount, entry_type) VALUES (?, ?, ?, ?)",
        pay_period_id,
        label,
        amount,
        entry_type
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_budget_entry(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM budget_entries WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}
