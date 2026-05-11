use crate::errors::AppError;
use axum::extract::Multipart;
use maud::{html, Markup};
use crate::features::transactions::models::{normalize_description, UsaaCsv};

pub(crate) async fn import(mut multipart: Multipart) -> Result<Markup, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("multipart error: {e}")))?
    {
        if field.name() != Some("csv") {
            continue;
        }

        let filename = field.file_name().unwrap_or("unknown").to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("read error: {e}")))?;

        tracing::info!(filename = %filename, size = bytes.len(), "received csv");

        let mut reader = csv::Reader::from_reader(bytes.as_ref());
        let mut count = 0;
        let mut errors = 0;

        for result in reader.deserialize::<UsaaCsv>() {
            match result {
                Ok(row) => {
                    if row.category.as_deref() == Some("Transfer") {
                        tracing::debug!(?row, "skipping transfer");
                        continue;
                    }
                    let normalized = normalize_description(&row.original_description);
                    tracing::info!(?row, %normalized, "parsed row");
                    count += 1;
                }
                Err(e) => {
                    tracing::warn!(error = %e, "failed to parse row");
                    errors += 1;
                }
            }
        }

        return Ok(html! {
            div .notification.is-success {
                p { "Parsed " (count) " rows (" (errors) " errors). Check logs." }
            }
        });
    }

    Ok(html! {
        div .notification.is-warning { "No file received" }
    })
}