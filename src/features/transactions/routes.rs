use axum::Router;
use axum::routing::post;
use crate::features::transactions::handlers::import;
use crate::state::ApplicationState;

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route(
            "/transactions/import",
            post(import)
                .layer(tower_http::limit::RequestBodyLimitLayer::new(5 * 1024 * 1024)),
        )
}