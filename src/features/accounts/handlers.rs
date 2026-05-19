use crate::errors::AppError;
use crate::features::accounts::get_account_by_id;
use crate::features::accounts::models::{AccountCreationForm, ValuationForm};
use crate::features::accounts::queries::{
    get_account_summaries_for_user, insert_account, upsert_valuation,
};
use crate::features::accounts::templates::{
    render_account_dashboard, render_account_modal, render_accounts_table, render_valuation_modal,
};
use crate::features::auth::AuthSession;
use crate::shared::base::base_layout;
use crate::shared::currency::dollars_to_pennies;
use crate::state::ApplicationState;
use axum::Form;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Uri};
use axum::response::IntoResponse;
use maud::{Markup, html};

pub(crate) async fn accounts_dashboard(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    uri: Uri,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;
    let accounts = get_account_summaries_for_user(&state.pool, user_id).await?;
    Ok(base_layout(
        "Accounts",
        uri.path(),
        render_account_dashboard(&accounts),
    ))
}

pub(crate) async fn new_account() -> Result<Markup, AppError> {
    Ok(render_account_modal(None))
}

pub(crate) async fn create_account(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Form(form): Form<AccountCreationForm>,
) -> Result<axum::response::Response, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    let name = form.account_name.trim();
    if name.is_empty() || name.len() > 100 {
        return Ok(modal_with_error("Account name is required (max 100 chars)"));
    }

    let pennies = match dollars_to_pennies(&form.opening_balance_string) {
        Ok(pennies) => pennies,
        Err(_) => return Ok(modal_with_error("Invalid balance amount")),
    };

    let inserted = insert_account(
        &state.pool,
        user_id,
        form.account_name.trim(),
        form.bank,
        pennies,
    )
    .await?;

    // acount name already exists
    if !inserted {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Retarget", "#account-modal".parse().unwrap());
        headers.insert("HX-Reswap", "outerHTML".parse().unwrap());
        return Ok((
            headers,
            render_account_modal(Some("An account with that name already exists")),
        )
            .into_response());
    }

    // Success: delete the modal, swap table into container
    let accounts = get_account_summaries_for_user(&state.pool, user_id).await?;
    Ok(html! {
        div hx-swap-oob="delete:#account-modal" {}
        (render_accounts_table(&accounts))
    }
    .into_response())
}

pub(crate) async fn valuation_modal(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Path(account_id): Path<i64>,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    let account = get_account_by_id(&state.pool, user_id, account_id)
        .await?
        .ok_or(AppError::Forbidden)?;

    Ok(render_valuation_modal(account.id, &account.name, None))
}

pub(crate) async fn record_valuation(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Path(account_id): Path<i64>,
    Form(form): Form<ValuationForm>,
) -> Result<axum::response::Response, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    let account = get_account_by_id(&state.pool, user_id, account_id)
        .await?
        .ok_or(AppError::Forbidden)?;

    let pennies = match dollars_to_pennies(&form.balance_string) {
        Ok(p) => p,
        Err(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("HX-Retarget", "#valuation-modal".parse().unwrap());
            headers.insert("HX-Reswap", "outerHTML".parse().unwrap());
            return Ok((
                headers,
                render_valuation_modal(account.id, &account.name, Some("Invalid balance amount")),
            )
                .into_response());
        }
    };

    upsert_valuation(&state.pool, account_id, pennies).await?;

    let accounts = get_account_summaries_for_user(&state.pool, user_id).await?;
    Ok(html! {
    div hx-swap-oob="delete:#valuation-modal" {}
    (render_accounts_table(&accounts))
    }
    .into_response())
}

fn modal_with_error(message: &str) -> axum::response::Response {
    let mut headers = HeaderMap::new();
    headers.insert("HX-Retarget", "#account-modal".parse().unwrap());
    headers.insert("HX-Reswap", "outerHTML".parse().unwrap());
    (headers, render_account_modal(Some(message))).into_response()
}
