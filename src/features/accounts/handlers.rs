use crate::errors::AppError;
use crate::features::accounts::templates::{render_account_dashboard, render_account_modal};
use crate::shared::base::base_layout;
use axum::http::Uri;
use maud::Markup;

pub(crate) async fn accounts_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Accounts",
        uri.path(),
        render_account_dashboard(),
    ))
}

pub(crate) async fn new_account() -> Result<Markup, AppError> {
    Ok(render_account_modal())
}
