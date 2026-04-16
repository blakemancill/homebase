use maud::{html, Markup, PreEscaped, DOCTYPE};
use crate::templates::partials::render_sidebar;

pub fn base_layout(page_title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";

                // Sets theme to dark or light based on user preference
                script {
                    (PreEscaped("
                        const scheme = window.matchMedia('(prefers-color-scheme: dark)');
                        const apply = e => document.documentElement.setAttribute('data-bs-theme', e.matches ? 'dark' : 'light');
                        apply(scheme);
                        scheme.addEventListener('change', apply);
                    "))
                }

                // Bootstrap
                link rel="stylesheet" type="text/css" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css";
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js" {}

                // HTMX
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" {}

                title { (page_title) }
            }
            body .d-flex.flex-column {
                (render_sidebar())
                main .p3 #main-content {
                    (content)
                }
            }
        }
    }
}