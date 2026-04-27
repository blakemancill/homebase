use crate::shared::navbar::render_navbar;
use maud::{DOCTYPE, Markup, html};

pub fn base_layout(page_title: &str, current_path: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

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

                // Hyperscript
                script src="https://cdn.jsdelivr.net/npm/hyperscript.org@0.9.91/dist/_hyperscript.min.js" {}

                title { (page_title) }
            }
            body .is-flex.is-flex-direction-column hx-boost="true" {
                // hamburger + top bar only visible on mobile
                div .is-hidden-tablet {
                    (render_navbar(current_path))
                }
                div .columns.is-gapless {
                    // sidebar only visible on tablet+
                    div .column.is-2.is-hidden-mobile {
                        (render_navbar(current_path))
                    }
                    div .column {
                        main #main-content .p-5 style="min-height: 100vh;" {
                            (content)
                        }
                    }
                }
            }
        }
    }
}
