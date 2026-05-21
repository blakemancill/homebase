//! Feature modules where each owns one slice of the app.
//!
//! Every feature follows the same internal split (see the crate root): `routes`
//! wires paths to `handlers`, which call `queries` and render `templates`.
//!
//! - [`auth`]: login, sessions, and the password-checking backend.
//! - [`accounts`]: financial accounts and net-worth tracking.
//! - [`budget`]: zero-based paycheck budgeting (pay periods and entries).
//! - [`transactions`]: bank-CSV import and transaction history.
//! - [`home`]: the authenticated landing page.

pub mod accounts;
pub mod auth;
pub mod budget;
pub mod home;
pub mod transactions;
