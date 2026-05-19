use maud::{Markup, PreEscaped, html};

pub fn render_csv_upload_section() -> Markup {
    html! {
        form
            hx-post="/transactions/import"
            hx-encoding="multipart/form-data"
        {
            div .file.is-centered.is-boxed {
                label .file-label {
                    input .file-input
                        type="file" name="csv"
                        accept=".csv,text/csv"
                        required
                        _="on change trigger submit on closest <form/>"
                    {}
                    span .file-cta {
                        span .file-icon {
                            (PreEscaped(r#"
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                    stroke-width="1.5" stroke="currentColor" class="size-6">
                                  <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
                                </svg>
                            "#))
                        }
                        span .file-label { "Upload New Transactions" }
                    }
                }
            }
        }
    }
}

pub(crate) fn render_import_modal(account_id: i64, account_name: &str) -> Markup {
    html! {
        div #import-modal .modal.is-active
            _="on closeModal remove #import-modal"
        {
            div .modal-background _="on click trigger closeModal" {}
            div .modal-card {
                form
                    hx-post=(format!("/accounts/{}/transactions/import", account_id))
                    hx-target="#import-result"
                    hx-swap="innerHTML"
                    hx-encoding="multipart/form-data"
                {
                    header .modal-card-head {
                        p .modal-card-title { "Import to " (account_name) }
                        button .delete type="button" _="on click trigger closeModal" {}
                    }
                    section .modal-card-body {
                        div .file.is-centered.is-boxed.has-name {
                            label .file-label {
                                input .file-input
                                    type="file" name="csv"
                                    accept=".csv,text/csv"
                                    required
                                    _="on change
                                         if my.files.length > 0
                                           set #csv-filename.innerText to my.files[0].name
                                         end"
                                {}
                                span .file-cta {
                                    span .file-label { "Choose CSV..." }
                                }
                                span #csv-filename .file-name { "No file chosen" }
                            }
                        }
                        style { "#import-result:not(:empty) { margin-top: 1.5rem; }" }

                        div #import-result {}
                    }
                    footer .modal-card-foot {
                        div .buttons {
                            button .button.is-primary type="submit" { "Upload" }
                            button .button type="button" _="on click trigger closeModal" { "Close" }
                        }
                    }
                }
            }
        }
    }
}