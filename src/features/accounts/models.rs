use chrono::{NaiveDate, NaiveDateTime};

#[derive(sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub bank: Bank,
    pub opening_balance_pennies: i64,
    pub opening_date: NaiveDate,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Bank {
    Usaa,
    Ally,
    Fidelity,
}

impl Bank {
    pub const ALL: &'static [Bank] = &[Bank::Usaa, Bank::Ally, Bank::Fidelity];

    pub fn as_str(&self) -> &'static str {
        match self {
            Bank::Usaa => "usaa",
            Bank::Ally => "ally",
            Bank::Fidelity => "fidelity",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Bank::Usaa => "USAA",
            Bank::Ally => "Ally",
            Bank::Fidelity => "Fidelity",
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AccountCreationForm {
    pub account_name: String,
    pub bank: Bank,
    pub opening_balance_string: String,
}
