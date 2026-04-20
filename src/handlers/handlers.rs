use crate::errors::AppError;
use crate::templates::layout::base_layout;
use crate::templates::pages::budget_dashboard::render_budget_dashboard;
use crate::templates::pages::index::render_index;
use axum::http::Uri;
use maud::Markup;

pub async fn index(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout("Home", uri.path(), render_index()))
}

pub async fn budget_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Dashboard",
        uri.path(),
        render_budget_dashboard(),
    ))
}

pub async fn handle_404(uri: Uri) -> Result<Markup, AppError> {
    Err(AppError::NotFound(uri))
}
