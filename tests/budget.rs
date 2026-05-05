mod common;

use crate::common::{TestApp, body_string};
use axum::http::{Request, StatusCode};

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
async fn user_b_cannot_insert_into_user_a_pay_period() {
    let app = TestApp::spawn().await;
    let john_id = app.create_user("john").await;
    app.create_user("jane").await;
    let john_period_id = app
        .create_pay_period(john_id, "2026-01-01", "2026-01-15")
        .await;

    let jane = app.login_as("jane").await;
    let res = jane
        .post_form(
            "/budget-entry",
            &[
                ("entry_type", "income"),
                ("pay_period_id", &john_period_id.to_string()),
                ("label", "stolen"),
                ("amount", "9999.00"),
                ("start_date", "2026-01-01"),
                ("end_date", "2026-01-15"),
            ],
        )
        .await;

    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // verify nothing was inserted
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM budget_entries WHERE label = 'stolen'")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn invalid_amount_returns_form_with_htmx_retarget() {
    let app = TestApp::spawn().await;

    // Set up a pay period
    let jane_id = app.create_user("jane").await;
    let period_id = app
        .create_pay_period(jane_id, "2026-01-01", "2026-01-15")
        .await;
    let jane = app.login_as("jane").await;

    let res = jane
        .post_form(
            "/budget-entry",
            &[
                ("entry_type", "income"),
                ("pay_period_id", &period_id.to_string()),
                ("label", "salary"),
                ("amount", "not_a_number"),
                ("start_date", "2026-01-01"),
                ("end_date", "2026-01-15"),
            ],
        )
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("HX-Retarget").unwrap(), "#entry-form");
    assert_eq!(res.headers().get("HX-Reswap").unwrap(), "outerHTML");

    let body = body_string(res).await;
    assert!(body.contains("Invalid Amount"));
}

#[tokio::test]
async fn happy_path_create_period_and_entries() {
    let app = TestApp::spawn().await;

    // Set up a pay period
    let jane_id = app.create_user("jane").await;
    let period_id = app
        .create_pay_period(jane_id, "2026-01-01", "2026-01-15")
        .await;
    let jane = app.login_as("jane").await;

    jane.post_form(
        "/budget-entry",
        &[
            ("entry_type", "income"),
            ("pay_period_id", &period_id.to_string()),
            ("label", "salary"),
            ("amount", "3000.00"),
            ("start_date", "2026-01-01"),
            ("end_date", "2026-01-15"),
        ],
    )
    .await;

    let res = jane
        .post_form(
            "/budget-entry",
            &[
                ("entry_type", "expense"),
                ("pay_period_id", &period_id.to_string()),
                ("label", "rent"),
                ("amount", "1200.00"),
                ("start_date", "2026-01-01"),
                ("end_date", "2026-01-15"),
            ],
        )
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_string(res).await;

    // Returned fragment contains both entries and the remaining total
    assert!(body.contains("Salary"));
    assert!(body.contains("Rent"));
    assert!(body.contains("$1800.00")); // 3000 - 1200 remaining
}

#[tokio::test]
async fn authenticated_user_can_view_dashboard() {
    let app = TestApp::spawn().await;
    app.create_user("jane").await;
    let jane = app.login_as("jane").await;

    let res = jane.get("/dashboard").await;

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_string(res).await;
    // Pay period form should be present on initial dashboard load
    assert!(body.contains("Pay Period"));
}

#[tokio::test]
async fn authenticated_user_can_view_home() {
    let app = TestApp::spawn().await;
    app.create_user("john").await;
    let john = app.login_as("john").await;

    let res = john.get("/").await;

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_string(res).await;
    assert!(body.contains("Hello world!"));
}

#[tokio::test]
async fn user_can_delete_their_own_entry() {
    let app = TestApp::spawn().await;

    // Set up a pay period
    let jane_id = app.create_user("jane").await;
    let period_id = app
        .create_pay_period(jane_id, "2026-01-01", "2026-01-15")
        .await;
    let jane = app.login_as("jane").await;

    jane.post_form(
        "/budget-entry",
        &[
            ("entry_type", "income"),
            ("pay_period_id", &period_id.to_string()),
            ("label", "salary"),
            ("amount", "3000.00"),
            ("start_date", "2026-01-01"),
            ("end_date", "2026-01-15"),
        ],
    )
    .await;

    let entry_id: i64 = sqlx::query_scalar!("SELECT id FROM budget_entries WHERE label = 'salary'")
        .fetch_one(&app.pool)
        .await
        .expect("entry should exist");

    let res = jane
        .delete(&format!(
            "/budget-entry/delete?id={}&pay_period_id={}",
            entry_id, period_id
        ))
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let count: i64 = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "c!: i64" FROM budget_entries WHERE label = 'salary'"#
    )
    .fetch_one(&app.pool)
    .await
    .expect("count query");

    assert_eq!(count, 0);

    let body = body_string(res).await;
    // Empty list still renders the table shell with "Remaining"
    assert!(body.contains("Remaining"));
}

#[tokio::test]
async fn user_b_cannot_delete_user_a_entry() {
    let app = TestApp::spawn().await;
    app.create_user("john").await;

    // Set up a pay period
    let jane_id = app.create_user("jane").await;
    let period_id = app
        .create_pay_period(jane_id, "2026-01-01", "2026-01-15")
        .await;
    let jane = app.login_as("jane").await;

    jane.post_form(
        "/budget-entry",
        &[
            ("entry_type", "income"),
            ("pay_period_id", &period_id.to_string()),
            ("label", "salary"),
            ("amount", "3000.00"),
            ("start_date", "2026-01-01"),
            ("end_date", "2026-01-15"),
        ],
    )
    .await;

    let entry_id: i64 = sqlx::query_scalar!("SELECT id FROM budget_entries WHERE label = 'salary'")
        .fetch_one(&app.pool)
        .await
        .expect("entry exists");

    let john = app.login_as("john").await;
    let res = john
        .delete(&format!(
            "/budget-entry/delete?id={}&pay_period_id={}",
            entry_id, period_id
        ))
        .await;

    assert_eq!(res.status(), StatusCode::OK);

    let count: i64 = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "c!: i64" FROM budget_entries WHERE label = 'salary'"#
    )
    .fetch_one(&app.pool)
    .await
    .expect("count query");

    assert_eq!(
        count, 1,
        "john should not have been able to delete jane's entry"
    );
}
