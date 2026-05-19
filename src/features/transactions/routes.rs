use crate::features::transactions::handlers::{import, import_modal};
use crate::state::ApplicationState;
use axum::Router;
use axum::routing::{get, post};

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/accounts/{id}/import-modal", get(import_modal))
        .route(
            "/accounts/{id}/transactions/import",
            post(import).layer(tower_http::limit::RequestBodyLimitLayer::new(
                5 * 1024 * 1024,
            )),
        )
}
