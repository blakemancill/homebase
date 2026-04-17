use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use maud::html;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource Not Found")]
    NotFound,

    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        let body = html! {
            div .notification.is-danger {
                h2 { "Error" }
                p { (error_message) }
            }
        };

        (status, body).into_response()
    }
}