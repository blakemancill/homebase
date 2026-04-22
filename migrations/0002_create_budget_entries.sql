CREATE TABLE IF NOT EXISTS budget_entries (
    id INTEGER PRIMARY KEY,
    pay_period_id INTEGER NOT NULL REFERENCES pay_period,
    label TEXT NOT NULL,
    amount INTEGER NOT NULL,
    entry_type TEXT NOT NULL CHECK(entry_type IN ('income', 'expense'))
)