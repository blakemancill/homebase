use crate::shared::currency::{CurrencyError, decimal_to_pennies};
use chrono::NaiveDate;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum Status {
    Posted,
    Pending,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.to_lowercase().as_str() {
            "posted" => Ok(Status::Posted),
            "pending" => Ok(Status::Pending),
            other => Err(serde::de::Error::custom(format!("unknown status: {other}"))),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct UsaaCsv {
    pub date: NaiveDate,
    #[serde(rename = "Original Description")]
    pub original_description: String,
    pub category: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AllyCsv {
    #[serde(rename = "Date")]
    pub date: NaiveDate,
    #[serde(rename = "Amount", with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    #[serde(rename = "Description")]
    pub description: String,
}

pub(crate) struct ParsedTransaction {
    pub account_id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub raw_description: String,
    pub bank_category: Option<String>,
    pub amount_pennies: i64,
    pub status: Status,
}

impl ParsedTransaction {
    pub(crate) fn from_usaa(row: UsaaCsv, account_id: i64) -> Result<Self, CurrencyError> {
        Ok(Self {
            account_id,
            date: row.date,
            description: normalize_description(&row.original_description),
            raw_description: row.original_description,
            bank_category: row.category,
            amount_pennies: decimal_to_pennies(row.amount)?,
            status: row.status,
        })
    }

    pub(crate) fn from_ally(row: AllyCsv, account_id: i64) -> Result<Self, CurrencyError> {
        Ok(Self {
            account_id,
            date: row.date,
            description: normalize_description(&row.description),
            raw_description: row.description,
            bank_category: None,
            amount_pennies: decimal_to_pennies(row.amount)?,
            status: Status::Posted, // Ally exports only cleared transactions
        })
    }
}

static TRAILING_REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*\*+\d+\s*$").unwrap());
static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

pub(crate) fn normalize_description(s: &str) -> String {
    let stripped = TRAILING_REF.replace(s, "");
    WHITESPACE.replace_all(&stripped, " ").trim().to_uppercase()
}
