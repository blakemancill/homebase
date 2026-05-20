use chrono::{NaiveDate, NaiveDateTime};
use strum_macros::EnumIter;

#[derive(sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub bank: Bank,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Deserialize, serde::Serialize, EnumIter,
)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Bank {
    Usaa,
    Ally,
    Fidelity,
    HealthEquity,
    Inspira,
    CharlesSchwab,
}

pub enum ImportStrategy {
    Csv,
    ManualValuation,
}

impl Bank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Bank::Usaa => "usaa",
            Bank::Ally => "ally",
            Bank::Fidelity => "fidelity",
            Bank::HealthEquity => "healthequity",
            Bank::Inspira => "inspira",
            Bank::CharlesSchwab => "charlesschwab",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Bank::Usaa => "USAA",
            Bank::Ally => "Ally",
            Bank::Fidelity => "Fidelity",
            Bank::HealthEquity => "Health Equity",
            Bank::Inspira => "Inspira",
            Bank::CharlesSchwab => "Charles Schwab",
        }
    }

    pub fn import_strategy(&self) -> ImportStrategy {
        match self {
            Bank::Usaa | Bank::Ally => ImportStrategy::Csv,
            Bank::Fidelity | Bank::HealthEquity | Bank::Inspira | Bank::CharlesSchwab => {
                ImportStrategy::ManualValuation
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AccountCreationForm {
    pub account_name: String,
    pub bank: Bank,
    pub opening_balance_string: String,
}

#[derive(serde::Deserialize)]
pub struct ValuationForm {
    pub balance_string: String,
}

#[derive(sqlx::FromRow)]
pub struct AccountSummary {
    pub id: i64,
    pub name: String,
    pub bank: Bank,
    pub opening_balance_pennies: i64,
    pub opening_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub estimated_balance_pennies: i64,
}
