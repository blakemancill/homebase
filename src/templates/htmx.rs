use crate::templates::layout::base_layout;
use axum::http::{HeaderMap, Uri};
use maud::{html, Markup};
use crate::templates::partials::render_navbar;

// returns true if the request came from htmx
pub fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers.contains_key("hx-request")
}

// Wraps content with the base layout, unless the request comes from htmx
pub fn render_page_or_fragment(headers: &HeaderMap, uri: &Uri, title: &str, content: Markup) -> Markup {
    let current_path = uri.path();
    if is_htmx_request(headers) {
        html! {
            (content)
            (render_navbar(current_path))
        }
    } else {
        base_layout(title, current_path, content)
    }
}
