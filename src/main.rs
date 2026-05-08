use anyhow::Context;
use axum_login::tower_sessions::ExpiredDeletion;
use homebase::{build_app, state::ApplicationState};
use std::net::SocketAddr;
use tokio::signal;
use tokio::task::AbortHandle;
use tower_sessions_sqlx_store::SqliteStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = ApplicationState::new().await?;

    tracing::info!("database connected and migrations run");

    let session_store = SqliteStore::new(state.pool.clone());

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60)),
    );

    let app = build_app(state, true, true).await?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TCP listener")?;

    tracing::info!("listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
    .await
    .context("axum::serve failed")?;

    match deletion_task.await {
        Ok(result) => result?,
        Err(e) if e.is_cancelled() => {}
        Err(e) => return Err(e.into()),
    }
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
            Ok(mut s) => {
                s.recv().await;
            }
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
