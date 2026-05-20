use crate::errors::AppError;
use crate::features::accounts::{Bank, get_account_by_id};
use crate::features::auth::AuthSession;
use crate::features::transactions::models::{AllyCsv, UsaaCsv, parse_csv};
use crate::features::transactions::queries::insert_transactions_batch;
use crate::features::transactions::templates::render_import_modal;
use crate::state::ApplicationState;
use axum::body::Bytes;
use axum::extract::{Multipart, Path, State};
use maud::{Markup, html};

pub(crate) async fn import_modal(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Path(account_id): Path<i64>,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    let account = get_account_by_id(&state.pool, user_id, account_id)
        .await?
        .ok_or(AppError::Forbidden)?;

    Ok(render_import_modal(account.id, &account.name))
}

pub(crate) async fn import(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Path(account_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    // ownership check + fetch the account in one query
    let account = get_account_by_id(&state.pool, user_id, account_id)
        .await?
        .ok_or(AppError::Forbidden)?;

    let Some((filename, bytes)) = read_csv_field(&mut multipart).await? else {
        return Ok(html! {
            div .notification.is-warning { p { "No file received." } }
        });
    };

    tracing::info!(
        filename = %filename, size = bytes.len(), account_id,
        bank = ?account.bank, "received csv"
    );

    let summary = match account.bank {
        Bank::Usaa => parse_csv::<UsaaCsv>(bytes.as_ref(), account_id),
        Bank::Ally => parse_csv::<AllyCsv>(bytes.as_ref(), account_id),
        Bank::Fidelity | Bank::HealthEquity | Bank::Inspira | Bank::CharlesSchwab => {
            return Ok(html! {
                div .notification.is-warning {
                    p { "This account uses manual balance updates, not CSV import." }
                }
            });
        }
    };

    let total_parsed = summary.parsed.len() as u64;
    let new_rows = insert_transactions_batch(&state.pool, user_id, &summary.parsed).await?;
    let duplicates = total_parsed - new_rows;

    tracing::info!(
        new_rows,
        duplicates,
        skipped_pending = summary.skipped_pending,
        parse_errors = summary.parse_errors,
        "import complete"
    );

    Ok(html! {
        div .notification.is-success {
            p { strong { "Import complete." } }
            ul {
                li { (new_rows) " new transactions" }
                @if duplicates > 0              { li { (duplicates) " already imported (skipped)" } }
                @if summary.skipped_pending > 0 { li { (summary.skipped_pending) " pending (will import when posted)" } }
                @if summary.parse_errors > 0    { li .has-text-danger { (summary.parse_errors) " rows failed to parse — see logs" } }
            }
        }
    })
}

async fn read_csv_field(multipart: &mut Multipart) -> Result<Option<(String, Bytes)>, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("multipart error: {e}")))?
    {
        if field.name() == Some("csv") {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let bytes = field
                .bytes()
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("read error: {e}")))?;
            return Ok(Some((filename, bytes)));
        }
    }
    Ok(None)
}
