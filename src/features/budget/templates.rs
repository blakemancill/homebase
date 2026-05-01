use crate::features::budget::models::{Bar, BudgetEntry, EntryType, FormPrefill};
use chrono::NaiveDate;
use maud::{Markup, html};

pub(crate) fn render_budget_dashboard() -> Markup {
    html! {
        style {
            ".budget-column:has(#budget-view:empty) {
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
                                        then set #budget-view.innerHTML to ''
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
            div .column.budget-column {
                // Table
                div .table-container {
                    div #budget-view {}
                }
            }
        }
    }
}

pub(crate) fn render_budget_view(entries: &[BudgetEntry], pay_period_id: i64) -> Markup {
    html! {
        (render_waterfall(entries))
        (render_budget_table(entries, pay_period_id))
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
                                    hx-target="#budget-view"
                                    hx-swap="innerHTML"
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

pub(crate) fn render_entry_form(
    id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    prefill: Option<&FormPrefill>,
) -> Markup {
    html! {
       @let active = prefill.map(|p| &p.values.entry_type)
            .unwrap_or(&EntryType::Income);

        form
            hx-post="/budget-entry"
            hx-target="#budget-view"
            hx-swap="innerHTML"
            id="entry-form"
            _="on htmx:afterRequest[detail.successful] call me.reset() then send click to .peer-toggle.is-info" {
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

                    @if let Some(p) = prefill {
                        div .notification.is-danger { (p.error) }
                    }

                    // toggle buttons between income and expense
                    div .buttons.has-addons.is-centered {
                        button
                            .button.peer-toggle
                            .is-info[matches!(active, EntryType::Income)]
                            type="button"
                            autofocus[matches!(active, EntryType::Income)]
                            _=
                                r#"
                                    on click remove .is-danger from .peer-toggle
                                    then add .is-info to me
                                    then set #entry-type.value to 'income'
                                "#
                        { "Income" }

                        button
                            .button.peer-toggle
                            .is-danger[matches!(active, EntryType::Expense)]
                            autofocus[matches!(active, EntryType::Expense)]
                            type="button"
                            _=
                                r#"
                                    on click remove .is-info from .peer-toggle
                                    then add .is-danger to me
                                    then set #entry-type.value to 'expense'
                                "#
                        { "Expense" }
                    }
                    input #entry-type type="hidden" name="entry_type" value=(match active {
                        EntryType::Income => "income",
                        EntryType::Expense => "expense",
                    }) {}
                    input type="hidden" name="pay_period_id" value=(id) {}
                    input type="hidden" name="start_date" value=(start_date.to_string()) {}
                    input type="hidden" name="end_date" value=(end_date.to_string()) {}

                    // fields
                    div .field {
                        div .control {
                            label .label for="label" { "Label" }
                            input
                                .input type="text" name="label"
                                value=[prefill.map(|p| p.values.label.as_str())]
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

pub(crate) fn render_waterfall(entries: &[BudgetEntry]) -> Markup {
    // chart dimensions
    let width = 600.0;
    let height = 300.0;
    let padding_top = 20.0;
    let padding_bottom = 60.0;
    let padding_x = 40.0;
    let plot_height = height - padding_top - padding_bottom;
    let plot_width = width - 2.0 * padding_x;

    // bar builder
    // income adds, expense subtracts. remainder is final bar
    let mut bars: Vec<Bar> = Vec::with_capacity(entries.len() + 1);
    let mut running: i64 = 0;
    for entry in entries {
        let delta = match entry.entry_type {
            EntryType::Income => entry.amount,
            EntryType::Expense => -entry.amount,
        };
        let start = running;
        running += delta;
        bars.push(Bar {
            label: entry.label.clone(),
            start,
            end: running,
            kind: entry.entry_type.clone(),
        });
    }

    // final remainder bar
    bars.push(Bar {
        label: "Remaining".to_string(),
        start: 0,
        end: running,
        kind: if running >= 0 {
            EntryType::Income
        } else {
            EntryType::Expense
        },
    });

    // y-axis range
    let max_y = bars
        .iter()
        .map(|b| b.start.max(b.end))
        .max()
        .unwrap_or(0)
        .max(0);
    let min_y = bars
        .iter()
        .map(|b| b.start.min(b.end))
        .min()
        .unwrap_or(0)
        .min(0);
    let range = (max_y - min_y).max(1) as f64;

    // map penny value to coordinate
    let y_for =
        |pennies: i64| -> f64 { padding_top + plot_height * (max_y - pennies) as f64 / range };

    let bar_width = plot_width / bars.len() as f64 * 0.6;
    let bar_step = plot_width / bars.len() as f64;
    let zero_y = y_for(0);

    html! {
        div .card.p-3 {
            svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox=(format!("0 0 {} {}", width, height))
                style="width: 100%; height: auto;"
            {
                // zero baseline
                line
                    x1=(padding_x) y1=(zero_y)
                    x2=(padding_x + plot_width) y2=(zero_y)
                    stroke="currentColor" stroke-width="1" opacity="0.3" {}

                @for (i, bar) in bars.iter().enumerate() {
                    @let x = padding_x + bar_step * i as f64 + (bar_step - bar_width) / 2.0;
                    @let y_top = y_for(bar.start.max(bar.end));
                    @let y_bot = y_for(bar.start.min(bar.end));
                    @let h = (y_bot - y_top).max(1.0);
                    @let fill = match bar.kind {
                        EntryType::Income => "#48c78e",
                        EntryType::Expense => "#f14668",
                    };

                    rect
                        x=(x) y=(y_top)
                        width=(bar_width) height=(h)
                        fill=(fill) rx="2" {}

                    // dollar label above each bar
                    text
                        x=(x + bar_width / 2.0)
                        y=(y_top - 4.0)
                        text-anchor="middle"
                        font-size="11"
                        fill="currentColor"
                    {
                        (format_pennies(bar.end - bar.start))
                    }

                    // category label below the chart
                    text
                        x=(x + bar_width / 2.0)
                        y=(height - padding_bottom + 16.0)
                        text-anchor="middle"
                        font-size="11"
                        fill="currentColor"
                    {
                        (title_case(&bar.label))
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
