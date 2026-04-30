pub mod errors;
pub mod features;
pub mod shared;
pub mod state;

use crate::state::ApplicationState;
use anyhow::Context;
use axum::Router;
use axum_login::{login_required, AuthManagerLayerBuilder};
use axum_login::tower_sessions::{Expiry, SessionManagerLayer, ExpiredDeletion};
use sqlx::SqlitePool;
use time::Duration;
use tokio::signal;
use tokio::task::AbortHandle;
use tower_http::trace::TraceLayer;
use tower_sessions_sqlx_store::SqliteStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = ApplicationState::new().await?;
    bootstrap_user(&state.pool).await?;
    return Ok(());
    tracing::info!("database connected and migrations run");

    let session_store = SqliteStore::new(state.pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let backend = features::auth::Backend::new(state.pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer.clone()).build();

    let protected = Router::new()
        .merge(features::home::routes())
        .merge(features::budget::routes())
        .route_layer(login_required!(features::auth::Backend, login_url = "/login"));

    let app = Router::new()
        .merge(protected)
        .merge(features::auth::routes())
        .fallback(errors::handle_404)
        .with_state(state)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await
        .context("failed to bind TCP listener")?;
    tracing::info!("listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .context("axum::serve failed")?;

    deletion_task.await??;
    Ok(())
}

async fn bootstrap_user(pool: &SqlitePool) -> anyhow::Result<()> {
    let username = std::env::var("BOOTSTRAP_USERNAME")
        .context("BOOTSTRAP_USERNAME must be set")?;
    let password = std::env::var("BOOTSTRAP_PASSWORD")
        .context("BOOTSTRAP_PASSWORD must be set")?;

    let hash = password_auth::generate_hash(&password);

    sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        username,
        hash,
    )
        .execute(pool)
        .await?;

    tracing::info!("bootstrapped user {username}");
    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        if let Err(e) = signal::ctrl_c().await {
            tracing::error!(?e, "failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut s) => { s.recv().await; }
            Err(e) => tracing::error!(?e, "failed to install SIGTERM handler"),
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => deletion_task_abort_handle.abort(),
        _ = terminate => deletion_task_abort_handle.abort(),
    }

    tracing::info!("shutting down...");
}