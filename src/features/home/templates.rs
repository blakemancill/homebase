use maud::{Markup, html};

pub fn render_index() -> Markup {
    html! {
        h1 .title { "Welcome" }
        p { "Use the sidebar to navigate." }
    }
}
