#[derive(sqlx::FromRow)]
pub struct BudgetEntry {
    pub label: String,
    pub amount: i64,
    pub entry_type: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Income,
    Expense,
}

impl EntryType {
    pub fn as_str(&self) -> &str {
        match self {
            EntryType::Income => "income",
            EntryType::Expense => "expense",
        }
    }
}
