use maud::{Markup, html};

pub fn render_index() -> Markup {
    html! { h3 { "Hello world!" } }
}
