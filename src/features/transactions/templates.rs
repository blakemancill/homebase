use maud::{html, Markup};

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
