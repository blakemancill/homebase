pub mod errors;
pub mod handlers;
pub mod state;
pub mod templates;

use crate::handlers::budget_handlers::{budget_dashboard, create_pay_period};
use crate::handlers::handlers::{handle_404, index};
use crate::state::ApplicationState;
use anyhow::Context;
use axum::Router;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = ApplicationState::new().await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(budget_dashboard))
        .route("/pay-period", post(create_pay_period))
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
