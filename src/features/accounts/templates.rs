use maud::{html, Markup};

pub(crate) fn render_account_dashboard() -> Markup {
    html! {
        div .container.is-fluid {
            div .level {
                div .level-left { h1 .title { "Accounts" } }
                div .level-right {
                    button .button.is-primary { "Add New Account" }
                }
            }
            div .card {
                table .table.mx-auto {
                    thead {
                        th { "Name" }
                        th { "Bank" }
                        th { "Starting Balance" }
                        th { "Estimated Balance" }
                        th { "Opening Date" }
                        th { "Creation Date" }
                    }
                }
            }
        }
    }
}