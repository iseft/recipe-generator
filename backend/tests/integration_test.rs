use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
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
