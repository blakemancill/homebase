use rust_decimal::Decimal;

#[derive(Debug, thiserror::Error)]
pub enum CurrencyError {
    #[error("invalid amount: {0}")]
    InvalidAmount(#[from] rust_decimal::Error),
    #[error("amount out of range")]
    AmountOutOfRange(#[from] std::num::TryFromIntError),
}

pub fn decimal_to_pennies(d: Decimal) -> Result<i64, CurrencyError> {
    let pennies = (d * Decimal::from(100)).round();
    Ok(pennies.try_into()?)
}

pub fn dollars_to_pennies(s: &str) -> Result<i64, CurrencyError> {
    let d = s.parse::<Decimal>()?;
    decimal_to_pennies(d)
}

pub fn format_pennies(pennies: i64) -> String {
    let sign = if pennies < 0 { "-" } else { "" };
    let abs = pennies.unsigned_abs();
    format!("{sign}${}.{:02}", abs / 100, abs % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_pennies_positive() {
        assert_eq!(format_pennies(1050), "$10.50");
    }

    #[test]
    fn format_pennies_negative() {
        assert_eq!(format_pennies(-1050), "-$10.50");
    }

    #[test]
    fn format_pennies_negative_less_than_dollar() {
        assert_eq!(format_pennies(-5), "-$0.05");
    }
}
