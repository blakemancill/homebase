#![allow(dead_code)]

pub mod budget;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode, header};
use axum::response::Response;
use homebase::build_app;
use homebase::state::ApplicationState;
use serde::Serialize;
use sqlx::SqlitePool;
use std::sync::OnceLock;
use tempfile::TempDir;
use tower::util::ServiceExt;

/// one password for all users
static TEST_PASSWORD_HASH: OnceLock<String> = OnceLock::new();

pub const TEST_PASSWORD: &str = "password";

fn test_password_hash() -> &'static str {
    TEST_PASSWORD_HASH.get_or_init(|| password_auth::generate_hash(TEST_PASSWORD))
}

pub struct TestApp {
    pub pool: SqlitePool,
    router: Router,
    _tmpdir: TempDir,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let tmpdir = TempDir::new().expect("failed to create tmp dir");
        let db_path = tmpdir.path().join("test.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        let state = ApplicationState::from_url(&db_url)
            .await
            .expect("failed to create application state");

        let pool = state.pool.clone();

        let router = build_app(state, false, false)
            .await
            .expect("failed to build application router");

        Self {
            pool,
            router,
            _tmpdir: tmpdir,
        }
    }

    /// creates a new user directly and returns that users ID
    pub async fn create_user(&self, username: &str) -> i64 {
        let hash = test_password_hash();
        sqlx::query_scalar!(
            "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id",
            username,
            hash
        )
        .fetch_one(&self.pool)
        .await
        .expect("failed to create user")
    }

    /// login as a given user
    pub async fn login_as(&self, username: &str) -> AuthedClient {
        let body =
            serde_urlencoded::to_string([("username", username), ("password", TEST_PASSWORD)])
                .unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap();

        let res = self.router.clone().oneshot(req).await.unwrap();

        // login redirects on success, fail if that doesn't happen
        assert_eq!(
            res.status(),
            StatusCode::SEE_OTHER,
            "login did not redirect: wrong creds, or secure cookie mismatch"
        );

        let cookie = res
            .headers()
            .get_all(header::SET_COOKIE)
            .iter()
            .find_map(|v| {
                let s = v.to_str().ok()?;
                // tower-sessions default cookie name is "id"
                if s.starts_with("id=") {
                    // strip attributes after the first ';'
                    Some(s.split(';').next().unwrap().to_string())
                } else {
                    None
                }
            })
            .expect("set-cookie with session id");

        AuthedClient {
            router: self.router.clone(),
            cookie,
        }
    }

    /// Unauthenticated request
    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}

pub struct AuthedClient {
    router: Router,
    cookie: String,
}

impl AuthedClient {
    pub async fn get(&self, uri: &str) -> Response<Body> {
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .header(header::COOKIE, &self.cookie)
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub async fn post_form<T: Serialize>(&self, uri: &str, form: &T) -> Response<Body> {
        let body = serde_urlencoded::to_string(form).unwrap();
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::COOKIE, &self.cookie)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(body))
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub async fn delete(&self, uri: &str) -> Response<Body> {
        let req = Request::builder()
            .method("DELETE")
            .uri(uri)
            .header(header::COOKIE, &self.cookie)
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }
}

/// Read a response body to a String
pub async fn body_string(res: Response<Body>) -> String {
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
