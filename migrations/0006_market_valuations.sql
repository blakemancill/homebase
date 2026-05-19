CREATE TABLE IF NOT EXISTS valuations (
    id INTEGER PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL DEFAULT (date('now')),
    value_pennies INTEGER NOT NULL,
    UNIQUE(account_id, date)
);

CREATE INDEX idx_valuations_account_date ON valuations(account_id, date DESC);