use crate::templates::layout::base_layout;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use maud::html;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource Not Found")]
    NotFound(Uri),

    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // Renders a 404 page on Not Found error
            AppError::NotFound(uri) => {
                tracing::warn!(path = %uri.path(), "404 not found");
                let content = html! {
                    div .notification.is-warning {
                        h1 { "404 - Page Not Found" }
                        br
                        p { "The page you're looking for doesn't exist." }
                    }
                };
                (
                    StatusCode::NOT_FOUND,
                    base_layout("Not Found", uri.path(), content),
                )
                    .into_response()
            }

            // Renders an error message on internal server error
            AppError::Internal(e) => {
                tracing::error!(error = %e, "Internal Server Error");
                let content = html! {
                    div .notification.is-danger {
                        h2 { "Error" }
                        br
                        p { "Internal Server Error" }
                    }
                };
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    base_layout("Error", "/", content),
                )
                    .into_response()
            }
        }
    }
}
