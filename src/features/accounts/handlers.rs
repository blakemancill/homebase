use axum::http::Uri;
use maud::Markup;
use crate::errors::AppError;
use crate::features::accounts::templates::render_account_dashboard;
use crate::shared::base::base_layout;

pub(crate) async fn budget_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Accounts",
        uri.path(),
        render_account_dashboard(),
    ))
}