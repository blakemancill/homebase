use crate::features::accounts::models::{Account, AccountSummary, Bank};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::SqlitePool;

pub(crate) async fn insert_account(
    pool: &SqlitePool,
    user_id: i64,
    name: &str,
    bank: Bank,
    opening_balance: i64,
) -> sqlx::Result<bool> {
    let was_inserted = sqlx::query!(
        r#"
            INSERT INTO accounts (user_id, name, bank, opening_balance_pennies)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(user_id, name) DO NOTHING
        "#,
        user_id,
        name,
        bank,
        opening_balance
    )
    .execute(pool)
    .await?;

    Ok(was_inserted.rows_affected() > 0)
}

pub(crate) async fn get_account_by_id(
    pool: &SqlitePool,
    user_id: i64,
    account_id: i64,
) -> sqlx::Result<Option<Account>> {
    sqlx::query_as!(
        Account,
        r#"
            SELECT
                id as "id!",
                name,
                bank as "bank: Bank"
            FROM accounts
            WHERE id = ? AND user_id = ?
        "#,
        account_id,
        user_id,
    )
        .fetch_optional(pool)
        .await
}

pub(crate) async fn get_account_summaries_for_user(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<Vec<AccountSummary>> {
    sqlx::query_as!(
        AccountSummary,
        r#"
            SELECT
                a.id as "id!",
                a.name,
                a.bank as "bank: Bank",
                a.opening_balance_pennies,
                a.opening_date as "opening_date: NaiveDate",
                a.created_at as "created_at: NaiveDateTime",
                a.opening_balance_pennies + COALESCE(
                    (SELECT SUM(amount_pennies)
                     FROM transactions
                     WHERE account_id = a.id
                       AND date >= a.opening_date),
                    0
                ) as "estimated_balance_pennies!: i64"
            FROM accounts a
            WHERE a.user_id = ?
            ORDER BY a.created_at DESC
        "#,
        user_id
    )
        .fetch_all(pool)
        .await
}