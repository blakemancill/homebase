mod common;

use axum::http::{Request, StatusCode};
use crate::common::TestApp;

#[tokio::test]
async fn unauthenticated_dashboard_redirects_to_login() {
    let app = TestApp::spawn().await;

    let req = Request::builder()
        .uri("/dashboard")
        .body(axum::body::Body::empty())
        .unwrap();

    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
    let location = res.headers().get("location").unwrap().to_str().unwrap();
    assert!(location.starts_with("/login"));
}

#[tokio::test]
async fn authenticated_user_can_view_home() {
    let app = TestApp::spawn().await;
    app.create_user("john").await;
    let john = app.login_as("john").await;

    let res = john.get("/").await;

    assert_eq!(res.status(), StatusCode::OK);
}