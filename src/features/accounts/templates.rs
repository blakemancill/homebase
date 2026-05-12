use maud::{html, Markup};

pub(crate) fn render_account_dashboard() -> Markup {
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
                table #accounts-table .table.mx-auto {
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

pub(crate) fn render_account_modal() -> Markup {
    html! {
        div #account-modal .modal.is-active
            _="on closeModal remove #account-modal"
        {
            div .modal-background _="on click trigger closeModal" {}
            div .modal-card {
                form hx-post="/accounts" hx-target="#accounts-table" hx-swap="outerHTML"
                    _="on htmx:afterRequest trigger closeModal"
                {
                    header .modal-card-head {
                        p .modal-card-title { "New Account" }
                        button .delete type="button" _="on click trigger closeModal" {}
                    }
                    section .modal-card-body {
                        div .field {
                            label .label { "Account Name" }
                            div .control {
                                input .input type="text" placeholder="Account Name..." name="name";
                            }
                        }
                        div .field {
                            label .label { "Bank" }
                            div .control {
                                input .input type="text" placeholder="Bank..." name="bank";
                            }
                        }
                        div .field {
                            label .label { "Current Balance" }
                            div .control {
                                input .input type="text" placeholder="Balance..." name="balance";
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