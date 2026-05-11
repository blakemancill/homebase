use axum::Router;
use crate::state::ApplicationState;

pub fn routes() -> Router<ApplicationState> {
    Router::new()
}