use axum::http::HeaderMap;
use maud::{html, Markup};
use crate::templates::render_page_or_fragment;

pub async fn index(headers: HeaderMap) -> Markup {
    render_page_or_fragment(&headers, "Home", html! { h3 { "Hello world!" } })
}