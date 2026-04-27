use crate::features::home::handlers;
use crate::state::ApplicationState;
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router<ApplicationState> {
    Router::new().route("/", get(handlers::index))
}
