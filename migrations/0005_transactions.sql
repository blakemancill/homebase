CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL,
    description TEXT NOT NULL,
    raw_description TEXT NOT NULL,
    bank_category TEXT,
    amount_pennies INTEGER NOT NULL,
    status TEXT NOT NULL CHECK ( status IN ('posted', 'pending') ),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(account_id, date, raw_description, amount_pennies)
);

CREATE INDEX idx_transactions_user ON transactions(user_id);
CREATE INDEX idx_transactions_account_date ON transactions(account_id, date DESC);