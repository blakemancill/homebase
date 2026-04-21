use maud::{Markup, html};

pub fn render_budget_dashboard() -> Markup {
    html! {
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
                            button .card-footer-item.button.is-danger.mr-1 type="reset" { "Reset" }
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
