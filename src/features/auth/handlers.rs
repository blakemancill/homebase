use axum::Form;
use axum::http::Uri;
use axum::response::{IntoResponse, Redirect, Response};
use maud::Markup;
use crate::errors::AppError;
use crate::features::auth::AuthSession;
use crate::features::auth::models::Credentials;
use crate::features::auth::templates::render_login_page;
use crate::shared::base::base_layout;

pub(crate) async fn login_page(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout("Login", uri.path(), render_login_page()))
}

pub(crate) async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> Result<Response, AppError> {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::warn!("invalid credentials for {}", creds.username);
            let login_url = match &creds.next {
                Some(next) => format!("/login?next={next}"),
                None => "/login".to_string(),
            };
            return Ok(Redirect::to(&login_url).into_response());
        }
        Err(e) => return Err(AppError::Internal(e.into())),
    };

    auth_session
        .login(&user)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("login failed: {e}")))?;

    let target = creds.next.as_deref().unwrap_or("/");
    Ok(Redirect::to(target).into_response())
}