pub mod errors;
pub mod handlers;
pub mod templates;
pub mod state;

use crate::handlers::handlers::{budget_dashboard, handle_404, index};
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::state::ApplicationState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = ApplicationState::new().await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(budget_dashboard))
        .fallback(handle_404)
        .with_state(state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
