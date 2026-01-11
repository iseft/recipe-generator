use std::sync::Arc;

use crate::application::use_cases::{
    CreateShareUseCase, DeleteShareUseCase, GenerateRecipeUseCase, GetRecipeUseCase,
    ListRecipesUseCase, SaveRecipeUseCase,
};
use crate::domain::repositories::{RecipeRepository, RecipeShareRepository};
use crate::domain::services::LlmService;

pub struct AppState<
    T: LlmService + 'static,
    R: RecipeRepository + 'static,
    S: RecipeShareRepository + 'static,
> {
    pub generate_use_case: Arc<GenerateRecipeUseCase<T>>,
    pub save_use_case: Arc<SaveRecipeUseCase<R>>,
    pub get_use_case: Arc<GetRecipeUseCase<R>>,
    pub list_use_case: Arc<ListRecipesUseCase<R>>,
    pub create_share_use_case: Arc<CreateShareUseCase<R, S>>,
    pub delete_share_use_case: Arc<DeleteShareUseCase<R, S>>,
}

impl<T: LlmService + 'static, R: RecipeRepository + 'static, S: RecipeShareRepository + 'static>
    Clone for AppState<T, R, S>
{
    fn clone(&self) -> Self {
        Self {
            generate_use_case: Arc::clone(&self.generate_use_case),
            save_use_case: Arc::clone(&self.save_use_case),
            get_use_case: Arc::clone(&self.get_use_case),
            list_use_case: Arc::clone(&self.list_use_case),
            create_share_use_case: Arc::clone(&self.create_share_use_case),
            delete_share_use_case: Arc::clone(&self.delete_share_use_case),
        }
    }
}
