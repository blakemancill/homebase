use crate::templates::render_page_or_fragment;
use axum::http::{HeaderMap, Uri};
use maud::{Markup, html};

pub async fn index(headers: HeaderMap, uri: Uri) -> Markup {
    render_page_or_fragment(
        &headers,
        &uri,
        "Home",
        html! { h3 { "Hello world!" } }
    )
}
