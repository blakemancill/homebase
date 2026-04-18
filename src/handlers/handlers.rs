use crate::errors::AppError;
use crate::templates::pages::dashboard::render_dashboard;
use crate::templates::pages::index::render_index;
use crate::templates::render_page_or_fragment;
use axum::http::{HeaderMap, Uri};
use maud::Markup;

pub async fn index(headers: HeaderMap, uri: Uri) -> Result<Markup, AppError> {
    let content = render_index();
    Ok(render_page_or_fragment(&headers, &uri, "Home", content))
}

pub async fn dashboard(headers: HeaderMap, uri: Uri) -> Result<Markup, AppError> {
    let content = render_dashboard();
    Ok(render_page_or_fragment(
        &headers,
        &uri,
        "Dashboard",
        content,
    ))
}

pub async fn handle_404(headers: HeaderMap, uri: Uri) -> Result<Markup, AppError> {
    Err(AppError::NotFound(headers, uri))
}
