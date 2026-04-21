use chrono::NaiveDate;
use maud::{Markup, html};

pub fn render_entry_form(id: i64, start_date: NaiveDate, end_date: NaiveDate) -> Markup {
    html! {
        form {
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
                                _="on invalid add .is-danger to me"
                            {}
                        }
                    }
                    div .field {
                        div .control {
                            label .label for="amount" { "Amount" }
                            input
                                .input type="text" name="amount" placeholder="0.00"
                                required
                                _="on invalid add .is-danger to me"
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
