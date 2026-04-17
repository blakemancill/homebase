use maud::{html, Markup};

pub fn render_index() -> Markup {
    html! { h3 { "Hello world!" } }
}
