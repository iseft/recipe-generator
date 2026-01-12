use std::sync::Arc;

use sqlx::PgPool;

use crate::recipes::application::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListOwnedRecipesUseCase, ListRecipeSharesUseCase, ListSharedRecipesUseCase, SaveRecipeUseCase,
};
use crate::recipes::infrastructure::{OpenAiClient, PgRecipeRepository, PgRecipeShareRepository};
use crate::shared::config::AppConfig;

pub struct RecipeDependencies {
    pub generate_use_case: Arc<GenerateRecipeUseCase<OpenAiClient>>,
    pub save_use_case: Arc<SaveRecipeUseCase<PgRecipeRepository>>,
    pub get_use_case: Arc<GetRecipeUseCase<PgRecipeRepository, PgRecipeShareRepository>>,
    pub list_owned_use_case: Arc<ListOwnedRecipesUseCase<PgRecipeRepository>>,
    pub list_shared_use_case: Arc<ListSharedRecipesUseCase<PgRecipeRepository>>,
    pub list_recipe_shares_use_case: Arc<ListRecipeSharesUseCase<PgRecipeShareRepository>>,
    pub create_share_use_case: Arc<CreateShareUseCase<PgRecipeRepository, PgRecipeShareRepository>>,
    pub delete_share_use_case: Arc<DeleteShareUseCase<PgRecipeRepository, PgRecipeShareRepository>>,
}

impl RecipeDependencies {
    pub fn new(config: &AppConfig, db_pool: PgPool) -> Self {
        let llm_client = Arc::new(OpenAiClient::new(config.openai_api_key.clone()));
        let recipe_repository = Arc::new(PgRecipeRepository::new(db_pool.clone()));
        let share_repository = Arc::new(PgRecipeShareRepository::new(db_pool));

        Self {
            generate_use_case: Arc::new(GenerateRecipeUseCase::new(llm_client)),
            save_use_case: Arc::new(SaveRecipeUseCase::new(recipe_repository.clone())),
            get_use_case: Arc::new(GetRecipeUseCase::new(
                recipe_repository.clone(),
                share_repository.clone(),
            )),
            list_owned_use_case: Arc::new(ListOwnedRecipesUseCase::new(recipe_repository.clone())),
            list_shared_use_case: Arc::new(ListSharedRecipesUseCase::new(
                recipe_repository.clone(),
            )),
            list_recipe_shares_use_case: Arc::new(ListRecipeSharesUseCase::new(
                share_repository.clone(),
            )),
            create_share_use_case: Arc::new(CreateShareUseCase::new(
                recipe_repository.clone(),
                share_repository.clone(),
            )),
            delete_share_use_case: Arc::new(DeleteShareUseCase::new(
                recipe_repository,
                share_repository,
            )),
        }
    }
}
