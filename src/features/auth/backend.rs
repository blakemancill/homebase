use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use sqlx::SqlitePool;
use tokio::task;
use crate::features::auth::models::{Credentials, User};

#[derive(Clone, Debug)]
pub struct Backend {
    db: SqlitePool,
}

impl Backend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Error> {
        let user: Option<Self::User> = sqlx::query_as!(
            User,
            r#"SELECT id as "id!", username, password_hash FROM users WHERE username = ?"#,
            creds.username
        )
        .fetch_optional(&self.db)
        .await?;

        // verify password via a task
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(creds.password, &user.password_hash).is_ok()))
        }).await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id as "id!", username, password_hash FROM users WHERE id = ?"#,
            user_id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(Into::into)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;