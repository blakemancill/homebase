use crate::features::accounts::models::{Account, Bank};
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

pub(crate) async fn get_accounts_for_user(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<Vec<Account>> {
    sqlx::query_as!(
        Account,
        r#"
            SELECT
                id as "id!",
                name,
                bank as "bank: Bank",
                opening_balance_pennies,
                opening_date as "opening_date: NaiveDate",
                created_at as "created_at: NaiveDateTime"
            FROM accounts
            WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
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
                bank as "bank: Bank",
                opening_balance_pennies,
                opening_date as "opening_date: NaiveDate",
                created_at as "created_at: NaiveDateTime"
            FROM accounts
            WHERE id = ? AND user_id = ?
        "#,
        account_id,
        user_id,
    )
        .fetch_optional(pool)
        .await
}

pub(crate) async fn account_belongs_to_user(
    pool: &SqlitePool,
    user_id: i64,
    account_id: i64,
) -> sqlx::Result<bool> {
    let count: i64 = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "c!: i64" FROM accounts WHERE id = ? AND user_id = ?"#,
        account_id,
        user_id,
    )
        .fetch_one(pool)
        .await?;
    Ok(count > 0)
}
