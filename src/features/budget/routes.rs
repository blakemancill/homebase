use axum::Router;
use axum::routing::{delete, get, post};
use crate::features::budget::handlers;
use crate::state::ApplicationState;

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/dashboard", get(handlers::budget_dashboard))
        .route("/pay-period", post(handlers::create_pay_period))
        .route("/budget-entry", post(handlers::create_budget_entry))
        .route("/budget-entry/delete", delete(handlers::delete_budget_entry))
}