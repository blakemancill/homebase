use crate::shared::currency::{CurrencyError, decimal_to_pennies};
use chrono::NaiveDate;
use regex::Regex;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
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
    pub(crate) account_id: i64,
    pub(crate) date: NaiveDate,
    pub(crate) description: String,
    pub(crate) raw_description: String,
    pub(crate) bank_category: Option<String>,
    pub(crate) amount_pennies: i64,
    pub(crate) status: Status,
}

static TRAILING_REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*\*+\d+\s*$").unwrap());
static WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

fn normalize_description(s: &str) -> String {
    let stripped = TRAILING_REF.replace(s, "");
    WHITESPACE.replace_all(&stripped, " ").trim().to_uppercase()
}

pub(crate) enum RowOutcome {
    Transaction(ParsedTransaction),
    SkippedPending,
}

pub(crate) trait BankCsv: DeserializeOwned {
    fn into_outcome(self, account_id: i64) -> Result<RowOutcome, CurrencyError>;
}

impl BankCsv for UsaaCsv {
    fn into_outcome(self, account_id: i64) -> Result<RowOutcome, CurrencyError> {
        if self.status == Status::Pending {
            return Ok(RowOutcome::SkippedPending);
        }
        Ok(RowOutcome::Transaction(ParsedTransaction {
            account_id,
            date: self.date,
            description: normalize_description(&self.original_description),
            raw_description: self.original_description,
            bank_category: self.category,
            amount_pennies: decimal_to_pennies(self.amount)?,
            status: self.status,
        }))
    }
}

impl BankCsv for AllyCsv {
    fn into_outcome(self, account_id: i64) -> Result<RowOutcome, CurrencyError> {
        Ok(RowOutcome::Transaction(ParsedTransaction {
            account_id,
            date: self.date,
            description: normalize_description(&self.description),
            raw_description: self.description,
            bank_category: None,
            amount_pennies: decimal_to_pennies(self.amount)?,
            status: Status::Posted, // ally only exports cleared transactions
        }))
    }
}

#[derive(Default)]
pub(crate) struct ParseSummary {
    pub(crate) parsed: Vec<ParsedTransaction>,
    pub(crate) parse_errors: u64,
    pub(crate) skipped_pending: u64,
}

pub(crate) fn parse_csv<T: BankCsv>(bytes: &[u8], account_id: i64) -> ParseSummary {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(bytes);

    let mut summary = ParseSummary::default();

    for result in reader.deserialize::<T>() {
        match result {
            Ok(row) => match row.into_outcome(account_id) {
                Ok(RowOutcome::Transaction(t)) => summary.parsed.push(t),
                Ok(RowOutcome::SkippedPending) => summary.skipped_pending += 1,
                Err(e) => {
                    tracing::warn!(error = %e, "amount conversion failed");
                    summary.parse_errors += 1;
                }
            },
            Err(e) => {
                tracing::warn!(error = %e, "csv row parse failed");
                summary.parse_errors += 1;
            }
        }
    }

    summary
}
