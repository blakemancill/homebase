CREATE TABLE IF NOT EXISTS pay_period (
    id INTEGER PRIMARY KEY,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    UNIQUE(start_date, end_date)
)