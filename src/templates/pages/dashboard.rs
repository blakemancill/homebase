use crate::templates::render_page_or_fragment;
use axum::http::{HeaderMap, Uri};
use maud::{Markup, html};

pub async fn render_dashboard(headers: HeaderMap, uri: Uri) -> Markup {
    render_page_or_fragment(
        &headers,
        &uri,
        "Dashboard",
        html! { h3 { "Dashboard" } }
    )
}
