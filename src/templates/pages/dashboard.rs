use crate::templates::render_page_or_fragment;
use axum::http::HeaderMap;
use maud::{html, Markup};

pub async fn render_dashboard(headers: HeaderMap) -> Markup {
    render_page_or_fragment(&headers, "Dashboard", html! { h3 { "Dashboard" } })
}