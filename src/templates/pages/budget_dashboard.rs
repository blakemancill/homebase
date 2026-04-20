use maud::{Markup, html};

pub fn render_budget_dashboard() -> Markup {
    html! {
        div .columns {
            div .column.is-one-third {
                // Form
                form {
                    div .card {
                        div .card-content {
                            // toggle buttons between income and expense
                            div .buttons.has-addons.is-centered {
                                button
                                    .button.is-info.peer-toggle type="button"
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

                            // fields
                            div .field {
                                div .control {
                                    label .label for="label" { "Label" }
                                    input .input type="text" name="label" autofocus
                                        placeholder="Label (e.g. Rent, Salary)" {}
                                }
                            }
                            div .field {
                                div .control {
                                    label .label for="amount" { "Amount" }
                                    input .input type="text" name="amount" placeholder="0.00" {}
                                }
                            }
                        }
                        div .card-footer.m-5.p-3 {
                            button .card-footer-item.button.is-info type="submit" { "Add Entry" }
                        }
                    }
                }
            }
            div .column.is-narrow {
                // Table
                div .card.p-3 {
                    table .table {
                        thead {
                            tr {
                                th { "Label" }
                                th { "Amount" }
                            }
                        }
                        tbody {

                        }
                    }
                }
            }
        }
    }
}
