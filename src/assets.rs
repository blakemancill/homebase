use crate::state::ApplicationState;
use axum::Router;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "public/"]
struct Assets;

pub fn routes() -> Router<ApplicationState> {
    Router::new()
        .route("/favicon.png", get(favicon))
        .route("/favicon-white.png", get(favicon_white))
        .route("/manifest.json", get(manifest))
}

async fn favicon() -> impl IntoResponse {
    serve_file("favicon.png")
}

async fn favicon_white() -> impl IntoResponse {
    serve_file("favicon-white.png")
}

async fn manifest() -> impl IntoResponse {
    serve_file("manifest.json")
}

fn serve_file(path: &str) -> Response {
    match Assets::get(path) {
        Some(file) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], file.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
