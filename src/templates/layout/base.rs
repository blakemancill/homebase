use crate::templates::partials::render_navbar;
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(page_title: &str, current_path: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";

                style {
                    "
                    :root {
                        --sidebar-bg: #d4ddd0;
                        --content-bg: #f9f9f7;
                    }
                    @media (prefers-color-scheme: dark) {
                        :root {
                            --sidebar-bg: #1a1a1a;
                            --content-bg: #242424;
                        }
                    }
                    #sidebar { background-color: var(--sidebar-bg); }
                    #main-content { background-color: var(--content-bg); }
                    "
                }

                // Bulma
                link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";

                // HTMX
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" {}

                title { (page_title) }
            }
            body .is-flex.is-flex-direction-column hx-boost="true" {
                div .columns.is-gapless style="min-height: 100vh;" {
                    div .column .is-2 {
                        (render_navbar(current_path))
                    }
                    div .column.is-flex.is-flex-direction-column {
                        main #main-content .p-5.is-flex-grow-1 {
                            (content)
                        }
                    }
                }
            }
        }
    }
}
