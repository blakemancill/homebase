use crate::features::auth::handlers::{login, login_page};
use crate::state::ApplicationState;
use axum::Router;
use axum::routing::{get, post};

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/login", get(login_page))
        .route("/login", post(login))
}
