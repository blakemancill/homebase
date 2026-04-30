use crate::features::budget::models::{BudgetEntry, EntryType};
use chrono::NaiveDate;
use sqlx::SqlitePool;

pub(crate) async fn get_entries_for_period(
    pool: &SqlitePool,
    user_id: i64,
    pay_period_id: i64,
) -> sqlx::Result<Vec<BudgetEntry>> {
    sqlx::query_as!(
        BudgetEntry,
        r#"
            SELECT id as "id!", label, amount, entry_type as "entry_type: EntryType"
            FROM budget_entries WHERE pay_period_id = ?
            AND user_id = ?
       "#,
        pay_period_id,
        user_id,
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
    user_id: i64,
    start: NaiveDate,
    end: NaiveDate,
) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO pay_period (user_id, start_date, end_date) VALUES (?, ?, ?)
            ON CONFLICT (user_id, start_date, end_date) DO UPDATE SET start_date = pay_period.start_date
            returning id
        "#,
        user_id, start, end
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn insert_budget_entry(
    pool: &SqlitePool,
    user_id: i64,
    pay_period_id: i64,
    label: &str,
    amount: i64,
    entry_type: EntryType,
) -> sqlx::Result<bool> {
    let was_inserted = sqlx::query!(
        r#"
            INSERT INTO budget_entries (user_id, pay_period_id, label, amount, entry_type)
            SELECT ?, ?, ?, ?, ?
            WHERE EXISTS (
                SELECT 1 FROM pay_period
                WHERE id = ? AND user_id = ?
            )
        "#,
        user_id,
        pay_period_id,
        label,
        amount,
        entry_type,
        pay_period_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(was_inserted.rows_affected() > 0)
}

pub(crate) async fn remove_budget_entry(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM budget_entries WHERE id = ? AND user_id = ?",
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
