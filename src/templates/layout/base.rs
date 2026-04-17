use crate::templates::partials::render_navbar;
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(page_title: &str, current_path: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";

                // Bulma
                link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";

                // HTMX
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" {}

                title { (page_title) }
            }
            body .is-flex.is-flex-direction-column {
                (render_navbar(current_path))
                main .p-3 #main-content {
                    (content)
                }
            }
        }
    }
}
