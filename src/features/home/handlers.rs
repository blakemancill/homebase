use crate::errors::AppError;
use crate::features::home::templates::render_index;
use axum::http::Uri;
use maud::Markup;
use crate::shared::base::base_layout;

pub async fn index(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout("Home", uri.path(), render_index()))
}