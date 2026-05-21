//! Application error type and its HTML responses.

use crate::shared::base::base_layout;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use maud::{Markup, html};
use thiserror::Error;

/// Router fallback for unmatched paths.
///
/// Always returns [`AppError::NotFound`]. The `IntoResponse` impl turns it into
/// the 404 page. Wired up via [`axum::Router::fallback`].
pub async fn handle_404(uri: Uri) -> Result<Markup, AppError> {
    Err(AppError::NotFound(uri))
}

/// The crate-wide error type returned by handlers.
///
/// Implements [`IntoResponse`] so that handlers can convert errors into rendered
/// HTML error pages.
#[derive(Debug, Error)]
pub enum AppError {
    /// Requested path didn't match any route.
    #[error("Resource Not Found")]
    NotFound(Uri),

    /// User isn't permitted to access the resource.
    #[error("Forbidden")]
    Forbidden,

    /// Unexpected internal failure. Renders a 500 page, and the underlying error isn't shown
    /// to the user
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),

    /// Database error. Same handling as [`Internal`](Self::Internal).
    #[error("Database Error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
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

            AppError::Internal(e) => internal_error(e.to_string()),
            AppError::Database(e) => internal_error(e.to_string()),

            AppError::Forbidden => {
                tracing::warn!("403 Forbidden");
                let content = html! {
                    div .notification.is-danger {
                        h1 { "403 - Forbidden" }
                        br;
                        p { "You don't have permission to perform that action." }
                    }
                };
                (
                    StatusCode::FORBIDDEN,
                    base_layout("Forbidden", "/", content),
                )
                    .into_response()
            }
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
