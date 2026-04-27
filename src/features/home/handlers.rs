use crate::errors::AppError;
use crate::features::home::templates::render_index;
use crate::shared::base::base_layout;
use axum::http::Uri;
use maud::Markup;

pub async fn index(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout("Home", uri.path(), render_index()))
}
