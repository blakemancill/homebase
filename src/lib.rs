//! Homebase: a self-hosted personal finance dashboard.
//!
//! Built on a hypermedia-driven (HDA) stack. [axum] for routing, [maud] for compile-time
//! HTML, htmx for interactivity, and sqlx over SQLite for storage. There is no seperate frontend
//! build or JSON API, as handlers return only rendered HTML.
//!
//! # Architecture
//! Code is organized by feature under [`features`] (budget, transactions, accounts, auth, home).
//! Each feature follows the same split: `routes` wires paths to `handlers`, which call `queries`
//! (SQL) and `templates` (maud), passing `models` between them. Cross-cutting pieces live in
//! [`shared`], [`errors`], and [`state`].
//!
//! [`build_app`] assembles the full router. The binary in `main.rs` is a thin wrapper that
//! builds [`ApplicationState`] and serves it.
//!
//! [axum]: https://docs.rs/axum
//! [maud]: https://docs.rs/maud

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

/// Assembles the application router with session, auth, and rate-limit layers.
///
/// # Arguments
/// * `secure_cookies`: when true, session cookies are marked as secure, meaning HTTPS only. Pass
///     false for local HTTP development.
/// * `rate_limit`: when true, applies a per-IP rate limiter.
///
/// Returns the configured [`Router`], or an error if the session-store migration fails.
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

    // Protected routes redirect unauthorized requests to a login page.
    let protected = Router::new()
        .merge(features::home::routes())
        .merge(features::budget::routes())
        .merge(features::transactions::routes())
        .merge(features::accounts::routes())
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

        // tower_governor accumulates per-IP state forever, so we prune stale entries every 60s
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
