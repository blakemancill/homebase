use chrono::NaiveDate;

#[derive(sqlx::FromRow)]
pub(crate) struct BudgetEntry {
    pub id: i64,
    pub label: String,
    pub amount: i64,
    pub entry_type: EntryType,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub(crate) enum EntryType {
    Income,
    Expense,
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Income => write!(f, "income"),
            EntryType::Expense => write!(f, "expense"),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct PayPeriodForm {
    pub(crate) start_date: NaiveDate,
    pub(crate) end_date: NaiveDate,
}

#[derive(serde::Deserialize)]
pub(crate) struct BudgetEntryForm {
    pub entry_type: EntryType,
    pub(crate) pay_period_id: i64,
    pub label: String,
    pub(crate) amount: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub(crate) struct FormPrefill<'a> {
    pub values: &'a BudgetEntryForm,
    pub error: &'a str,
}

#[derive(serde::Deserialize)]
pub(crate) struct DeleteBudgetEntryForm {
    pub(crate) id: i64,
    pub(crate) pay_period_id: i64,
}

// Helpers
#[derive(Debug, thiserror::Error)]
pub(crate) enum BudgetError {
    #[error("invalid amount: {0}")]
    InvalidAmount(#[from] rust_decimal::Error),
    #[error("amount out of range")]
    AmountOutOfRange(#[from] std::num::TryFromIntError),
}

pub(crate) struct Bar {
    pub label: String,
    pub start: i64,
    pub end: i64,
    pub kind: EntryType,
}
