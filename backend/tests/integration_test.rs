use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tower::util::ServiceExt;
use uuid::Uuid;

mod common;

async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://recipe_user:recipe_password@localhost:5432/recipe_generator_test".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test database pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query("DELETE FROM recipes")
        .execute(pool)
        .await
        .expect("Failed to cleanup test data");
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_save_and_list_recipes_require_auth() {
    let _pool = create_test_pool().await;

    let recipe_data = serde_json::json!({
        "title": "Test Recipe",
        "ingredients": ["chicken", "rice"],
        "instructions": ["Cook chicken", "Add rice"],
        "prepTimeMinutes": 10,
        "cookTimeMinutes": 20,
        "servings": 4
    });

    let save_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/recipes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&recipe_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(save_response.status(), StatusCode::UNAUTHORIZED);

    let list_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .uri("/api/recipes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_recipe_by_id_requires_auth() {
    let _pool = create_test_pool().await;
    let fake_id = Uuid::new_v4();

    let get_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .uri(&format!("/api/recipes/{}", fake_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_shared_recipes_requires_auth() {
    let _pool = create_test_pool().await;

    let response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .uri("/api/recipes/shared")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_generate_requires_auth() {
    let _pool = create_test_pool().await;

    let request_body = serde_json::json!({
        "ingredients": ["chicken", "rice"]
    });

    let response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/recipes/generate")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_share_endpoints_require_auth() {
    let _pool = create_test_pool().await;
    let fake_id = Uuid::new_v4();

    let share_request = serde_json::json!({
        "userId": "user_123"
    });

    let create_share_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/recipes/{}/shares", fake_id))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&share_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_share_response.status(), StatusCode::UNAUTHORIZED);

    let delete_share_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/api/recipes/{}/shares/user_123", fake_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_share_response.status(), StatusCode::UNAUTHORIZED);
}
