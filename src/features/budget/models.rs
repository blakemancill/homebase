#[derive(sqlx::FromRow)]
pub struct BudgetEntry {
    pub id: i64,
    pub label: String,
    pub amount: i64,
    pub entry_type: EntryType,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
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
