use axum::Router;
use axum_login::{login_required, AuthManagerLayerBuilder};
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use time::Duration;
use tower_http::trace::TraceLayer;
use tower_sessions_sqlx_store::SqliteStore;
use crate::state::ApplicationState;

pub mod errors;
pub mod features;
pub mod shared;
pub mod state;

pub async fn build_app(state: ApplicationState, secure_cookies: bool) -> anyhow::Result<Router> {
    let session_store = SqliteStore::new(state.pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookies)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let backend = features::auth::Backend::new(state.pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer.clone()).build();

    let protected = Router::new()
        .merge(features::home::routes())
        .merge(features::budget::routes())
        .route_layer(login_required!(
            features::auth::Backend,
            login_url = "/login"
        ));

    Ok(Router::new()
        .merge(protected)
        .merge(features::auth::routes())
        .fallback(errors::handle_404)
        .with_state(state)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http()))
}