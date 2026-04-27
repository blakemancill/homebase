DROP TABLE IF EXISTS budget_entries;
DROP TABLE IF EXISTS pay_period;

CREATE TABLE users (
   id INTEGER PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   password_hash TEXT NOT NULL,
   created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE pay_period (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    UNIQUE(user_id, start_date, end_date)
);

CREATE TABLE budget_entries (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    pay_period_id INTEGER NOT NULL REFERENCES pay_period(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    amount INTEGER NOT NULL,
    entry_type TEXT NOT NULL CHECK(entry_type IN ('income', 'expense'))
);

CREATE INDEX idx_pay_period_user ON pay_period(user_id);
CREATE INDEX idx_budget_entries_user ON budget_entries(user_id);
CREATE INDEX idx_budget_entries_pay_period ON budget_entries(pay_period_id);