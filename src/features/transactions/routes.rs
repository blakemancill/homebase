use crate::features::transactions::handlers::import;
use crate::state::ApplicationState;
use axum::Router;
use axum::routing::post;

pub fn routes() -> Router<ApplicationState> {
    Router::new().route(
        "/transactions/import",
        post(import).layer(tower_http::limit::RequestBodyLimitLayer::new(
            5 * 1024 * 1024,
        )),
    )
}
