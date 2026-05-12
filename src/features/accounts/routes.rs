use axum::Router;
use axum::routing::get;
use crate::features::accounts::handlers::{budget_dashboard, new_account};
use crate::state::ApplicationState;

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/accounts", get(budget_dashboard))
        .route("/account-modal", get(new_account))
}