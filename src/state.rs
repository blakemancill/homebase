//! Shared application state handed to every request handler.

use anyhow::Context;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::str::FromStr;

/// Resources shared by all request handlers.
///
/// Handed to the router with [`axum::Router::with_state`] and extracted in handlers with
/// [`axum::extract::State`]. Deriving [`Clone`] is required by axum, which clones the state into
/// each handler.
#[derive(Clone)]
pub struct ApplicationState {
    /// Shared SQLite connection pool.
    pub pool: SqlitePool,
}

impl ApplicationState {
    /// Builds state from the `DATABASE_URL` env variable.
    ///
    /// Thin wrapper over [`from_url`](Self::from_url).
    ///
    /// # Errors
    /// If `DATABASE_URL` is unset, or if connection or migration fails.
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        Self::from_url(&db_url).await
    }

    /// Opens a pooled connection to `db_url` and runs any pending migrations
    ///
    /// # Errors
    /// if `db_url` is malformed, the connection can't be opening, or a migration fails.
    pub async fn from_url(db_url: &str) -> anyhow::Result<Self> {
        let options = SqliteConnectOptions::from_str(db_url)?
            // turns on sqlite foreign key support
            .foreign_keys(true)
            // allows many readers but only one writer
            .journal_mode(SqliteJournalMode::Wal)
            // wait for up to 5 seconds on a locked DB instead of erroring out
            .busy_timeout(std::time::Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // runs everything in /migrations at startup
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}
