use crate::features::budget::models::{BudgetEntry, EntryType};
use chrono::NaiveDate;
use maud::{Markup, html};

pub(crate) fn render_budget_dashboard() -> Markup {
    html! {
        style {
            ".column.is-narrow:has(#budget-table:empty) {
                display: none;
            }
            input[type='date']::-webkit-calendar-picker-indicator {
                filter: invert(1);
                opacity: 0;
                position: absolute;
                right: 0;
                width: 100%;
                cursor: pointer;
            }
            input[type='date'] {
                position: relative;
            }
            "
        }
        div .columns {
            div .column.is-one-third {
                // pay period input form
                form hx-post="/pay-period" hx-target="#entry-form" hx-swap="innerHTML" {
                    div .card {
                        div .card-content {
                            div .field {
                                label { "Pay Period" }
                                div .control {
                                    input
                                        .input type="date" name="start_date" autofocus required
                                        _="on invalid add .is-danger to me on input remove .is-danger from me"
                                    {}
                                }
                                span .span { "To: "}
                                div .control {
                                    input
                                        .input type="date" name="end_date" required
                                        _="on invalid add .is-danger to me on input remove .is-danger from me"
                                    {}
                                }
                            }
                        }
                        div .card-footer.m-5.p-3 {
                            button
                                .card-footer-item.button.is-danger.mr-1
                                type="reset"
                                _=
                                    r#"
                                        on click set #entry-form.innerHTML to ''
                                        then set #budget-table.innerHTML to ''
                                        then add .is-hidden to .column.is-narrow
                                    "#
                            { "Reset" }
                            button .card-footer-item.button.is-primary type="submit" { "Submit" }
                        }
                    }
                }
                br;
                // Data Entry Form
                div #entry-form {}
            }
            div .column.is-narrow {
                // Table
                div .table-container {
                    div #budget-table {}
                }
            }
        }
    }
}

pub(crate) fn render_budget_table(entries: &[BudgetEntry], pay_period_id: i64) -> Markup {
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
        div #budget-table .card.p-3 {
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

pub(crate) fn render_entry_form(id: i64, start_date: NaiveDate, end_date: NaiveDate) -> Markup {
    html! {
        form
            hx-post="/budget-entry"
            hx-target="#budget-table"
            hx-swap="outerHTML"
            _="on htmx:afterRequest call me.reset() then send click to .peer-toggle.is-info" {
            div .card {
                div .card-content {
                    div .card-header {
                        p .card-header-title.is-centered {
                            (format!(
                                "{} to {}",
                                start_date.format("%B %d, %Y"),
                                end_date.format("%B %d, %Y")
                            ))
                        }
                    }
                    br;
                    // toggle buttons between income and expense
                    div .buttons.has-addons.is-centered {
                        button
                            .button.is-info.peer-toggle type="button"
                            autofocus
                            _=
                                r#"
                                    on click remove .is-danger from .peer-toggle
                                    then add .is-info to me
                                    then set #entry-type.value to 'income'
                                "#
                        { "Income" }

                        button
                            .button.peer-toggle type="button"
                            _=
                                r#"
                                    on click remove .is-info from .peer-toggle
                                    then add .is-danger to me
                                    then set #entry-type.value to 'expense'
                                "#
                        { "Expense" }
                    }
                    input #entry-type type="hidden" name="entry_type" value="income" {}
                    input type="hidden" name="pay_period_id" value=(id) {}

                    // fields
                    div .field {
                        div .control {
                            label .label for="label" { "Label" }
                            input
                                .input type="text" name="label"
                                placeholder="Label (e.g. Rent, Salary)"
                                required
                                _="on invalid add .is-danger to me on input remove .is-danger from me"
                            {}
                        }
                    }
                    div .field {
                        div .control {
                            label .label for="amount" { "Amount" }
                            input
                                .input type="text" name="amount" placeholder="0.00"
                                required
                                _="on invalid add .is-danger to me on input remove .is-danger from me"
                            {}
                        }
                    }
                }
                div .card-footer.m-5.p-3 {
                    button .card-footer-item.button.is-info type="submit" { "Add Entry" }
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
