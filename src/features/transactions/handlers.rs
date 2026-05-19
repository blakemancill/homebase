use crate::errors::AppError;
use crate::features::accounts::get_account_by_id;
use crate::features::auth::AuthSession;
use crate::features::transactions::models::{ParsedTransaction, Status, UsaaCsv};
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

    let mut csv_bytes: Option<Bytes> = None;
    let mut filename = String::from("unknown");

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("multipart error: {e}")))?
    {
        if field.name() == Some("csv") {
            filename = field.file_name().unwrap_or("unknown").to_string();
            let bytes = field
                .bytes()
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("read error: {e}")))?;
            csv_bytes = Some(bytes);
        }
    }

    let Some(bytes) = csv_bytes else {
        return Ok(html! {
            div .notification.is-warning { p { "No file received." } }
        });
    };

    tracing::info!(
        filename = %filename, size = bytes.len(), account_id,
        bank = ?account.bank, "received csv"
    );

    let mut parsed: Vec<ParsedTransaction> = Vec::new();
    let mut parse_errors: u64 = 0;
    let mut skipped_pending: u64 = 0;

    let mut reader = csv::Reader::from_reader(bytes.as_ref());
    for result in reader.deserialize::<UsaaCsv>() {
        match result {
            Ok(row) => {
                if row.status == Status::Pending {
                    skipped_pending += 1;
                    continue;
                }
                match ParsedTransaction::from_usaa(row, account_id) {
                    Ok(p) => parsed.push(p),
                    Err(e) => {
                        tracing::warn!(error = %e, "amount conversion failed");
                        parse_errors += 1;
                    }
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "csv row parse failed");
                parse_errors += 1;
            }
        }
    }

    let total_parsed = parsed.len() as u64;
    let new_rows = insert_transactions_batch(&state.pool, user_id, &parsed).await?;
    let duplicates = total_parsed - new_rows;

    tracing::info!(
        new_rows,
        duplicates,
        skipped_pending,
        parse_errors,
        "import complete"
    );

    Ok(html! {
        div .notification.is-success {
            p { strong { "Import complete." } }
            ul {
                li { (new_rows) " new transactions" }
                @if duplicates > 0      { li { (duplicates) " already imported (skipped)" } }
                @if skipped_pending > 0 { li { (skipped_pending) " pending (will import when posted)" } }
                @if parse_errors > 0    { li .has-text-danger { (parse_errors) " rows failed to parse — see logs" } }
            }
        }
    })
}
