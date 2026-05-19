use crate::features::accounts::handlers::{
    accounts_dashboard, create_account, new_account, record_valuation, valuation_modal,
};
use crate::state::ApplicationState;
use axum::Router;
use axum::routing::{get, post};

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/accounts", get(accounts_dashboard))
        .route("/account-modal", get(new_account))
        .route("/accounts", post(create_account))
        .route("/accounts/{id}/valuation-modal", get(valuation_modal))
        .route("/accounts/{id}/valuation", post(record_valuation))
}
