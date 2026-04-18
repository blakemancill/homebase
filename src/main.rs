pub mod templates;
pub mod handlers;
pub mod errors;

use crate::handlers::handlers::{dashboard, handle_404, index};
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(dashboard))
        .fallback(handle_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
