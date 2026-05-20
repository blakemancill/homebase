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
                b.estimated_balance_pennies as "estimated_balance_pennies!: i64"
            FROM accounts a
            JOIN account_balances b ON b.account_id = a.id
            WHERE a.user_id = ?
            ORDER BY a.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn upsert_valuation(
    pool: &SqlitePool,
    account_id: i64,
    value_pennies: i64,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO valuations (account_id, value_pennies)
            VALUES (?, ?)
            ON CONFLICT (account_id, date) DO UPDATE SET value_pennies = excluded.value_pennies
        "#,
        account_id,
        value_pennies
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_net_worth_for_user(pool: &SqlitePool, user_id: i64) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
            SELECT COALESCE(SUM(estimated_balance_pennies), 0) AS "net_worth!: i64"
            FROM account_balances
            WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}
