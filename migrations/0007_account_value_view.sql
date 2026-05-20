CREATE VIEW account_balances AS
SELECT
    a.id      AS account_id,
    a.user_id AS user_id,
    COALESCE(
            (SELECT v.value_pennies
             FROM valuations v
             WHERE v.account_id = a.id
             ORDER BY v.date DESC
             LIMIT 1),
            a.opening_balance_pennies + COALESCE(
                    (SELECT SUM(t.amount_pennies)
                     FROM transactions t
                     WHERE t.account_id = a.id
                       AND t.date >= a.opening_date),
                    0
            )
    ) AS estimated_balance_pennies
FROM accounts a;