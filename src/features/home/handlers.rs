use crate::errors::AppError;
use crate::features::accounts::get_net_worth_for_user;
use crate::features::auth::AuthSession;
use crate::features::home::templates::render_index;
use crate::shared::base::base_layout;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::Uri;
use maud::Markup;

pub async fn index(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    uri: Uri,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    let net_worth = get_net_worth_for_user(&state.pool, user_id).await?;

    Ok(base_layout("Home", uri.path(), render_index(net_worth)))
}
