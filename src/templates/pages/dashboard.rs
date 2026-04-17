use crate::templates::render_page_or_fragment;
use axum::http::HeaderMap;
use maud::{Markup, html};

pub async fn render_dashboard(headers: HeaderMap) -> Markup {
    render_page_or_fragment(
        &headers, 
        "Dashboard", 
        "/dashboard", 
        html! { h3 { "Dashboard" } }
    )
}
