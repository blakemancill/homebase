use crate::features::accounts::models::{AccountSummary, Bank};
use crate::shared::currency::format_pennies;
use maud::{Markup, html};

pub(crate) fn render_account_dashboard(accounts: &[AccountSummary]) -> Markup {
    html! {
        div .container.is-fluid {
            div .level {
                div .level-left { h1 .title { "Accounts" } }
                div .level-right {
                    button .button.is-primary
                        hx-get="/account-modal"
                        hx-target="body"
                        hx-swap="beforeend"
                    { "Add New Account" }
                }
            }
            div .card {
                (render_accounts_table(accounts))
            }
        }
    }
}

pub(crate) fn render_accounts_table(accounts: &[AccountSummary]) -> Markup {
    html! {
        table #accounts-table .table.mx-auto {
            thead {
                tr {
                    th { "Name" }
                    th { "Bank" }
                    th { "Starting Balance" }
                    th { "Estimated Balance" }
                    th { "Opening Date" }
                    th { "Creation Date" }
                    th { "Actions" }
                }
            }
            tbody {
                @for account in accounts {
                    tr {
                        td { (account.name) }
                        td { (account.bank.display_name()) }
                        td { (format_pennies(account.opening_balance_pennies)) }
                        td { (format_pennies(account.estimated_balance_pennies)) }
                        td { (account.opening_date.format("%Y-%m-%d")) }
                        td { (account.created_at.format("%Y-%m-%d")) }
                        td {
                            button .button.is-small.is-info
                                hx-get=(format!("/accounts/{}/import-modal", account.id))
                                hx-target="body"
                                hx-swap="beforeend"
                            { "Import CSV" }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn render_account_modal(error: Option<&str>) -> Markup {
    html! {
        div #account-modal .modal.is-active
            _="on closeModal remove #account-modal"
        {
            div .modal-background _="on click trigger closeModal" {}
            div .modal-card {
                form hx-post="/accounts" hx-target="#accounts-table" hx-swap="outerHTML"
                {
                    header .modal-card-head {
                        p .modal-card-title { "New Account" }
                        button .delete type="button" _="on click trigger closeModal" {}
                    }
                    section .modal-card-body {
                        @if let Some(error) = error {
                            div .notification.is-danger { (error)}
                        }
                        div .field {
                            label .label { "Account Name" }
                            div .control {
                                input .input
                                    type="text"
                                    required autofocus
                                    placeholder="Account Name..."
                                    name="account_name";
                            }
                        }
                        div .field {
                            label .label { "Bank" }
                            div .select.is-fullwidth {
                                select name="bank" {
                                    @for bank in Bank::ALL {
                                        option value=(bank.as_str()) { (bank.display_name()) }
                                    }
                                }
                            }
                        }
                        div .field {
                            label .label { "Current Balance" }
                            div .control {
                                input .input type="text" required placeholder="Balance..." name="opening_balance_string";
                            }
                        }
                    }
                    footer .modal-card-foot {
                        div .buttons {
                            button .button.is-success type="submit" { "Save" }
                            button .button type="button" _="on click trigger closeModal" { "Cancel" }
                        }
                    }
                }
            }
        }
    }
}
