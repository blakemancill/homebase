pub mod errors;
pub mod features;
pub mod shared;
pub mod state;

use crate::state::ApplicationState;
use anyhow::Context;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = ApplicationState::new().await?;
    tracing::info!("database connected and migrations run");

    let app = Router::new()
        .merge(features::home::routes())
        .merge(features::budget::routes())
        .fallback(errors::handle_404)
        .with_state(state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    tracing::info!("listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum::serve failed")?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl+c");
    tracing::info!("shutting down...");
}
