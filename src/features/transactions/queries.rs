use sqlx::SqlitePool;
use crate::features::transactions::models::ParsedTransaction;

pub(crate) async fn insert_transactions_batch(
    pool: &SqlitePool,
    user_id: i64,
    parsed: &[ParsedTransaction],
) -> sqlx::Result<u64> {
    let mut txn = pool.begin().await?;
    let mut new_rows: u64 = 0;

    for t in parsed {
        let result = sqlx::query!(
            r#"
                INSERT INTO transactions
                    (user_id, account_id, date, description, raw_description,
                     bank_category, amount_pennies, status)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT (account_id, date, raw_description, amount_pennies) DO NOTHING
            "#,
            user_id,
            t.account_id,
            t.date,
            t.description,
            t.raw_description,
            t.bank_category,
            t.amount_pennies,
            t.status,
        )
        .execute(&mut *txn)
        .await?;

        new_rows += result.rows_affected();
    }

    txn.commit().await?;
    Ok(new_rows)
}
