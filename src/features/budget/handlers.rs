use crate::errors::AppError;
use crate::features::budget::models::EntryType;
use crate::features::budget::queries::{
    get_entries_for_period, insert_budget_entry, remove_budget_entry, upsert_pay_period,
};
use crate::features::budget::templates::{
    render_budget_dashboard, render_budget_table, render_entry_form,
};
use crate::shared::base::base_layout;
use crate::state::ApplicationState;
use axum::Form;
use axum::extract::{Query, State};
use axum::http::Uri;
use chrono::NaiveDate;
use maud::{Markup, html};
use rust_decimal::Decimal;
use crate::features::auth::AuthSession;

pub(crate) async fn budget_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Dashboard",
        uri.path(),
        render_budget_dashboard(),
    ))
}

pub(crate) async fn create_pay_period(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Form(form): Form<PayPeriodForm>,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    // pay periods should make sense. You don't get paid backwards in time!
    if form.start_date >= form.end_date {
        return Ok(html! {
            div .notification.is-danger {
                "Start date must be before end date."
            }
        });
    }

    let id = upsert_pay_period(&state.pool, user_id, form.start_date, form.end_date).await?;
    let entries = get_entries_for_period(&state.pool, user_id, id).await?;

    Ok(html! {
        // primary: goes into hx-target #entry-form
        (render_entry_form(id, form.start_date, form.end_date))

        // oob swap: htmx puts this into #budget-table
        div #budget-table hx-swap-oob="outerHTML" {
            (render_budget_table(&entries, id))
        }
    })
}

pub(crate) async fn create_budget_entry(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Form(form): Form<BudgetEntryForm>,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;

    // convert amount into pennies for accuracy
    let pennies = dollars_to_pennies(&form.amount).map_err(|e| AppError::Internal(e.into()))?;

    let inserted = insert_budget_entry(
        &state.pool,
        user_id,
        form.pay_period_id,
        &form.label,
        pennies,
        form.entry_type,
    )
    .await?;

    if !inserted { return Err(AppError::Forbidden) }

    let entries = get_entries_for_period(&state.pool, user_id, form.pay_period_id).await?;

    Ok(render_budget_table(&entries, form.pay_period_id))
}

pub(crate) async fn delete_budget_entry(
    auth_session: AuthSession,
    State(state): State<ApplicationState>,
    Query(form): Query<DeleteBudgetEntryForm>,
) -> Result<Markup, AppError> {
    let user_id = auth_session.user.ok_or(AppError::Forbidden)?.id;
    remove_budget_entry(&state.pool, user_id, form.id).await?;
    let entries = get_entries_for_period(&state.pool, user_id, form.pay_period_id).await?;
    Ok(render_budget_table(&entries, form.pay_period_id))
}

// Form shapes
#[derive(serde::Deserialize)]
pub(crate) struct PayPeriodForm {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[derive(serde::Deserialize)]
pub(crate) struct BudgetEntryForm {
    entry_type: EntryType,
    pay_period_id: i64,
    label: String,
    amount: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct DeleteBudgetEntryForm {
    id: i64,
    pay_period_id: i64,
}

// Helpers
#[derive(Debug, thiserror::Error)]
enum BudgetError {
    #[error("invalid amount: {0}")]
    InvalidAmount(#[from] rust_decimal::Error),
    #[error("amount out of range")]
    AmountOutOfRange(#[from] std::num::TryFromIntError),
}

fn dollars_to_pennies(s: &str) -> Result<i64, BudgetError> {
    let d = s.parse::<Decimal>()?;
    let pennies = (d * Decimal::from(100)).round();
    Ok(pennies.try_into()?)
}
