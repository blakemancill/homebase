use crate::models::BudgetEntry;
use maud::{Markup, html};

pub fn render_budget_table(entries: &[BudgetEntry], oob: bool) -> Markup {
    let total_income: i64 = entries
        .iter()
        .filter(|e| e.entry_type == "income")
        .map(|e| e.amount)
        .sum();

    let total_expenses: i64 = entries
        .iter()
        .filter(|e| e.entry_type == "expense")
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
                                span .tag.is-primary[entry.entry_type == "income"]
                                     .is-danger[entry.entry_type == "expense"]
                                {
                                    (title_case(entry.entry_type.as_str()))
                                }
                            }
                            td { (format_pennies(entry.amount)) }
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
    format!("${:.2}", pennies as f64 / 100.0)
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
