use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use maud::html;
use thiserror::Error;
use crate::templates::render_page_or_fragment;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource Not Found")]
    NotFound(HeaderMap, Uri),

    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // Renders a 404 page on Not Found error
            AppError::NotFound(headers, uri) => {
                let content = html! {
                    div .notification.is-warning {
                        h1 { "404 - Page Not Found" }
                        br
                        p { "The page you're looking for doesn't exist." }
                    }
                };
                let page = render_page_or_fragment(&headers, &uri, "Not Found", content);
                (StatusCode::NOT_FOUND, page).into_response()
            },

            // Renders an error message on internal server error
            AppError::Internal(_) => {
                let body = html! {
                    div .notification.is-danger {
                        h2 { "Error" }
                        br
                        p { "Internal Server Error" }
                    }
                };
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}