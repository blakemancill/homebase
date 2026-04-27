use crate::features::budget::models::{BudgetEntry, EntryType};
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub(crate) async fn get_entries_for_period(
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

/// Returns the id of the pay period for (start, end), inserting one if absent
///
/// The 'DO UPDATE SET' is deliberate no-op
/// 'ON CONFLICT DO NOTHING' would skip the row and the return would be nothing.
/// so by forcibly writing, we guarantee an id comes back
pub(crate) async fn upsert_pay_period(
    pool: &SqlitePool,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO pay_period (start_date, end_date) VALUES (?, ?)
            ON CONFLICT (start_date, end_date) DO UPDATE SET start_date = pay_period.start_date
            returning id
        "#,
        start, end
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn insert_budget_entry(
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

pub(crate) async fn remove_budget_entry(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM budget_entries WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(())
}
