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
