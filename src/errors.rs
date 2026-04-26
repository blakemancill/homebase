use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use maud::{html, Markup};
use thiserror::Error;
use crate::shared::base::base_layout;

pub async fn handle_404(uri: Uri) -> Result<Markup, AppError> {
    Err(AppError::NotFound(uri))
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource Not Found")]
    NotFound(Uri),

    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),

    #[error("Database Error")]
    Database(#[from] sqlx::Error),
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
            AppError::Internal(e) => internal_error(e.to_string()),
            AppError::Database(e) => internal_error(e.to_string()),
        }
    }
}

fn internal_error(e: impl std::fmt::Display) -> Response {
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
