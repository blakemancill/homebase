#[derive(sqlx::FromRow)]
pub struct BudgetEntry {
    pub label: String,
    pub amount: i64,
    pub entry_type: String,
}