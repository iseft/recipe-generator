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
async fn test_save_and_list_recipes() {
    let pool = create_test_pool().await;
    cleanup_test_data(&pool).await;

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

    assert_eq!(save_response.status(), StatusCode::OK);

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

    assert_eq!(list_response.status(), StatusCode::OK);

    let body = to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let recipes: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Find our specific recipe (tests run in parallel and may have other recipes)
    let our_recipe = recipes
        .iter()
        .find(|r| r["title"] == "Test Recipe")
        .expect("Should find our 'Test Recipe' in the list");

    assert_eq!(our_recipe["title"], "Test Recipe");
}

#[tokio::test]
async fn test_get_recipe_by_id() {
    let pool = create_test_pool().await;
    cleanup_test_data(&pool).await;

    let recipe_data = serde_json::json!({
        "title": "Get Test Recipe",
        "ingredients": ["pasta", "sauce"],
        "instructions": ["Boil pasta", "Add sauce"],
        "prepTimeMinutes": 5,
        "cookTimeMinutes": 15,
        "servings": 2
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

    assert_eq!(save_response.status(), StatusCode::OK);

    let save_body = to_bytes(save_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let saved_recipe: serde_json::Value = serde_json::from_slice(&save_body).unwrap();
    let recipe_id = saved_recipe["id"].as_str().unwrap();

    let get_response = common::create_test_app()
        .await
        .oneshot(
            Request::builder()
                .uri(&format!("/api/recipes/{}", recipe_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let get_body = to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let recipe: serde_json::Value = serde_json::from_slice(&get_body).unwrap();

    assert_eq!(recipe["id"], recipe_id);
    assert_eq!(recipe["title"], "Get Test Recipe");
}

#[tokio::test]
async fn test_get_nonexistent_recipe() {
    let app = common::create_test_app().await;
    let fake_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/recipes/{}", fake_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_error_not_exposed_to_client() {
    let detailed_error_msg = "Connection timeout to api.openai.com:443";
    let failing_llm = Arc::new(common::FailingLlmClient {
        error: backend::domain::services::LlmError::ApiError(detailed_error_msg.to_string()),
    });

    let app = common::create_test_app_with_llm(failing_llm).await;

    let request_body = serde_json::json!({
        "ingredients": ["chicken", "rice"]
    });

    let response = app
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

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_message = error_response["error"].as_str().unwrap();

    assert!(
        !error_message.contains(detailed_error_msg),
        "Detailed error message should not be exposed to client"
    );
    assert!(
        !error_message.contains("api.openai.com"),
        "Internal details should not be exposed to client"
    );

    assert!(
        error_message.contains("Failed to reach AI service"),
        "Should contain generic user-friendly message"
    );
}

#[tokio::test]
async fn test_parse_error_not_exposed_to_client() {
    let detailed_error_msg = "expected value at line 5 column 12: invalid JSON";
    let failing_llm = Arc::new(common::FailingLlmClient {
        error: backend::domain::services::LlmError::ParseError(detailed_error_msg.to_string()),
    });

    let app = common::create_test_app_with_llm(failing_llm).await;

    let request_body = serde_json::json!({
        "ingredients": ["chicken", "rice"]
    });

    let response = app
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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_message = error_response["error"].as_str().unwrap();

    assert!(
        !error_message.contains(detailed_error_msg),
        "Detailed error message should not be exposed to client"
    );
    assert!(
        !error_message.contains("line 5 column 12"),
        "Internal parsing details should not be exposed to client"
    );

    assert!(
        error_message.contains("Failed to process AI response"),
        "Should contain generic user-friendly message"
    );
}
