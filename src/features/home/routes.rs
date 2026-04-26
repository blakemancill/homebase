use axum::Router;
use axum::routing::get;
use crate::features::home::handlers;
use crate::state::ApplicationState;

pub fn routes() -> Router<ApplicationState> {
    Router::new().route("/", get(handlers::index))
}