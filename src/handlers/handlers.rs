use crate::errors::AppError;
use crate::state::ApplicationState;
use crate::templates::layout::base_layout;
use crate::templates::pages::budget_dashboard::render_budget_dashboard;
use crate::templates::pages::index::render_index;
use crate::templates::partials::render_entry_form;
use axum::extract::{Form, State};
use axum::http::Uri;
use chrono::NaiveDate;
use maud::{html, Markup};

pub async fn index(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout("Home", uri.path(), render_index()))
}

pub async fn budget_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Dashboard",
        uri.path(),
        render_budget_dashboard(),
    ))
}

pub async fn handle_404(uri: Uri) -> Result<Markup, AppError> {
    Err(AppError::NotFound(uri))
}

pub async fn create_pay_period(
    State(state): State<ApplicationState>,
    Form(form): Form<PayPeriodForm>,
) -> Result<Markup, AppError> {
    if form.start_date >= form.end_date {
        return Ok(html! {
            div .notification.is-danger {
                "Start date must be before end date."
            }
        });
    }

    sqlx::query(
        r#"
                INSERT OR IGNORE INTO pay_period (start_date, end_date) VALUES (?, ?)
            "#,
    )
    .bind(&form.start_date)
    .bind(&form.end_date)
    .execute(&state.pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    let id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM pay_period WHERE start_date = ? AND end_date = ?",
    )
    .bind(form.start_date)
    .bind(form.end_date)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(render_entry_form(id))
}

#[derive(serde::Deserialize)]
pub struct PayPeriodForm {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
