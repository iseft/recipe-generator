use utoipa::OpenApi;

use super::dto::{
    CreateShareRequest, GenerateRecipeRequest, GeneratedRecipeResponse, RecipeResponse,
    SaveRecipeRequest,
};
use super::handlers::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::recipes::adapters::handlers::generate_recipe,
        crate::recipes::adapters::handlers::save_recipe,
        crate::recipes::adapters::handlers::get_recipe,
        crate::recipes::adapters::handlers::list_my_recipes,
        crate::recipes::adapters::handlers::list_shared_recipes,
        crate::recipes::adapters::handlers::create_share,
        crate::recipes::adapters::handlers::delete_share,
    ),
    components(schemas(
        GenerateRecipeRequest,
        GeneratedRecipeResponse,
        SaveRecipeRequest,
        RecipeResponse,
        CreateShareRequest,
        ErrorResponse,
    )),
    tags(
        (name = "Recipes", description = "Recipe management endpoints"),
        (name = "Sharing", description = "Recipe sharing endpoints"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
