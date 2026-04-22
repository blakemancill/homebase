use crate::db::{get_entries_for_period, insert_budget_entry, upsert_pay_period};
use crate::errors::AppError;
use crate::state::ApplicationState;
use crate::templates::layout::base_layout;
use crate::templates::pages::budget_dashboard::render_budget_dashboard;
use crate::templates::partials::render_budget_table::render_budget_table;
use crate::templates::partials::render_entry_form;
use axum::Form;
use axum::extract::State;
use axum::http::Uri;
use chrono::NaiveDate;
use maud::{Markup, html};
use rust_decimal::Decimal;

pub async fn budget_dashboard(uri: Uri) -> Result<Markup, AppError> {
    Ok(base_layout(
        "Dashboard",
        uri.path(),
        render_budget_dashboard(),
    ))
}

pub async fn create_pay_period(
    State(state): State<ApplicationState>,
    Form(form): Form<PayPeriodForm>,
) -> Result<Markup, AppError> {
    // pay periods should make sense. You don't get paid backwards in time!
    if form.start_date >= form.end_date {
        return Ok(html! {
            div .notification.is-danger {
                "Start date must be before end date."
            }
        });
    }

    let id = upsert_pay_period(&state.pool, form.start_date, form.end_date).await?;
    let entries = get_entries_for_period(&state.pool, id).await?;

    Ok(html! {
        // primary: goes into hx-target #entry-form
        (render_entry_form(id, form.start_date, form.end_date))

        // oob swap: htmx puts this into #budget-table
        (render_budget_table(&entries, true))
    })
}

pub async fn create_budget_entry(
    State(state): State<ApplicationState>,
    Form(form): Form<BudgetEntryForm>,
) -> Result<Markup, AppError> {
    // convert amount into pennies for accuracy
    let pennies = dollars_to_pennies(&form.amount).map_err(|e| AppError::Internal(e.into()))?;

    insert_budget_entry(
        &state.pool,
        form.pay_period_id,
        &form.label,
        pennies,
        form.entry_type.as_str(),
    )
    .await?;

    let entries = get_entries_for_period(&state.pool, form.pay_period_id).await?;

    Ok(render_budget_table(&entries, false))
}

// Form shapes
#[derive(serde::Deserialize)]
pub struct PayPeriodForm {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[derive(serde::Deserialize)]
pub struct BudgetEntryForm {
    entry_type: EntryType,
    pay_period_id: i64,
    label: String,
    amount: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Income,
    Expense,
}

impl EntryType {
    fn as_str(&self) -> &str {
        match self {
            EntryType::Income => "income",
            EntryType::Expense => "expense",
        }
    }
}

// Helpers
#[derive(Debug, thiserror::Error)]
pub enum BudgetError {
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
