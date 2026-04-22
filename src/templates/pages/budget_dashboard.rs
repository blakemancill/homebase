use maud::{Markup, html};

pub fn render_budget_dashboard() -> Markup {
    html! {
        style {
            ".column.is-narrow:has(#budget-table:empty) {
                display: none;
            }"
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
                div #budget-table {}
            }
        }
    }
}
