use crate::models::{BudgetEntry, EntryType};
use maud::{Markup, html};

pub fn render_budget_table(entries: &[BudgetEntry], pay_period_id: i64, oob: bool) -> Markup {
    let total_income: i64 = entries
        .iter()
        .filter(|e| e.entry_type == EntryType::Income)
        .map(|e| e.amount)
        .sum();

    let total_expenses: i64 = entries
        .iter()
        .filter(|e| e.entry_type == EntryType::Expense)
        .map(|e| e.amount)
        .sum();

    let remaining = total_income - total_expenses;

    html! {
        div #budget-table .card.p-3 hx-swap-oob=[oob.then_some("outerHTML")] {
            table .table.is-fullwidth {
                thead {
                    tr {
                        th { "Label" }
                        th { "Type" }
                        th { "Amount" }
                    }
                }
                tbody {
                    @for entry in entries {
                        tr {
                            td { (title_case(entry.label.as_str())) }
                            td {
                                span .tag.is-primary[entry.entry_type == EntryType::Income]
                                     .is-danger[entry.entry_type == EntryType::Expense]
                                {
                                    (title_case(&entry.entry_type.to_string()))
                                }
                            }
                            td { (format_pennies(entry.amount)) }
                            td {
                                button .delete
                                    hx-delete="/budget-entry/delete"
                                    hx-target="#budget-table"
                                    hx-swap="outerHTML"
                                    hx-vals=(format!(r#"{{"id": {}, "pay_period_id": {}}}"#, entry.id, pay_period_id))
                                {}
                            }
                        }
                    }
                }
                tfoot {
                    tr {
                        th colspan="2" { "Remaining" }
                        th { (format_pennies(remaining)) }
                    }
                }
            }
        }
    }
}

fn format_pennies(pennies: i64) -> String {
    let sign = if pennies < 0 { "-" } else { "" };
    let abs = pennies.unsigned_abs();
    format!("{sign}${}.{:02}", abs / 100, abs % 100)
}

fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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

    #[test]
    fn title_case_basic() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn title_case_already_upper() {
        assert_eq!(title_case("HELLO"), "Hello");
    }

    #[test]
    fn title_case_lowercases_rest() {
        assert_eq!(title_case("hELLO wORLD"), "Hello World");
    }
}