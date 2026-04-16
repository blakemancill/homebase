pub mod templates;

use crate::templates::pages::dashboard::render_dashboard;
use anyhow::Context;
use axum::Router;
use axum::routing::get;
use templates::pages::index::index;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/dashboard", get(render_dashboard));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;

    axum::serve(listener, app)
        .await
        .context("axum::serve failed")?;

    Ok(())
}
