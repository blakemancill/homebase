use axum::http::HeaderMap;
use maud::Markup;
use crate::templates::layout::base_layout;

// returns true if the request came from htmx
pub fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers.contains_key("hx-request")
}

// Wraps content with the base layout, unless the request comes from htmx
pub fn render_page_or_fragment(headers: &HeaderMap, title: &str, content: Markup) -> Markup {
    if is_htmx_request(headers) {
        content
    } else {
        base_layout(title, content)
    }
}