mod recipes;
mod shared;

use std::net::SocketAddr;
use std::sync::Arc;

use recipes::adapters::create_router;
use recipes::application::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListOwnedRecipesUseCase, ListRecipeSharesUseCase, ListSharedRecipesUseCase, SaveRecipeUseCase,
};
use recipes::infrastructure::{OpenAiClient, PgRecipeRepository, PgRecipeShareRepository};
use shared::auth::init_clerk;
use shared::config::AppConfig;
use shared::db::create_pool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::from_filename("../.env")
        .or_else(|_| dotenvy::dotenv())
        .ok();

    let config = AppConfig::from_env();

    init_clerk(config.clerk_secret_key.clone());

    let db_pool = create_pool(&config.database_url).await;

    let llm_client = Arc::new(OpenAiClient::new(config.openai_api_key));
    let recipe_repository = Arc::new(PgRecipeRepository::new(db_pool.clone()));
    let share_repository = Arc::new(PgRecipeShareRepository::new(db_pool));

    let generate_use_case = Arc::new(GenerateRecipeUseCase::new(llm_client));
    let save_use_case = Arc::new(SaveRecipeUseCase::new(recipe_repository.clone()));
    let get_use_case = Arc::new(GetRecipeUseCase::new(
        recipe_repository.clone(),
        share_repository.clone(),
    ));
    let list_owned_use_case = Arc::new(ListOwnedRecipesUseCase::new(recipe_repository.clone()));
    let list_shared_use_case = Arc::new(ListSharedRecipesUseCase::new(recipe_repository.clone()));
    let list_recipe_shares_use_case =
        Arc::new(ListRecipeSharesUseCase::new(share_repository.clone()));
    let create_share_use_case = Arc::new(CreateShareUseCase::new(
        recipe_repository.clone(),
        share_repository.clone(),
    ));
    let delete_share_use_case =
        Arc::new(DeleteShareUseCase::new(recipe_repository, share_repository));

    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origin
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router(
        generate_use_case,
        save_use_case,
        get_use_case,
        list_owned_use_case,
        list_shared_use_case,
        list_recipe_shares_use_case,
        create_share_use_case,
        delete_share_use_case,
    )
    .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server running on http://localhost:{}", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
