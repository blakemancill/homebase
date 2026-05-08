use crate::state::ApplicationState;
use axum::Router;
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};
use time::Duration;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_http::trace::TraceLayer;
use tower_sessions_sqlx_store::SqliteStore;

mod assets;
pub mod errors;
pub mod features;
pub mod shared;
pub mod state;

pub async fn build_app(
    state: ApplicationState,
    secure_cookies: bool,
    rate_limit: bool,
) -> anyhow::Result<Router> {
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

    let mut app = Router::new()
        .merge(assets::routes())
        .merge(protected)
        .merge(features::auth::routes())
        .fallback(errors::handle_404)
        .with_state(state)
        .layer(auth_layer)
        .layer(session_layer);

    if rate_limit {
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(20)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap();

        let limiter = governor_conf.limiter().clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                limiter.retain_recent();
            }
        });

        app = app.layer(GovernorLayer::new(governor_conf));
    }

    Ok(app.layer(TraceLayer::new_for_http()))
}
