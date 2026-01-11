use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use clerk_rs::validators::authorizer::ClerkJwt;
use std::sync::OnceLock;

pub use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
};

static CLERK_CLIENT: OnceLock<Clerk> = OnceLock::new();

pub fn init_clerk(secret_key: String) {
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let clerk = Clerk::new(config);
    CLERK_CLIENT.set(clerk).ok();
}

pub fn get_clerk() -> Option<&'static Clerk> {
    CLERK_CLIENT.get()
}

pub fn create_clerk_layer() -> ClerkLayer<MemoryCacheJwksProvider> {
    let clerk = get_clerk().expect("Clerk not initialized");
    ClerkLayer::new(MemoryCacheJwksProvider::new(clerk.clone()), None, true)
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jwt = parts
            .extensions
            .get::<ClerkJwt>()
            .ok_or((StatusCode::UNAUTHORIZED, "Not authenticated"))?;

        Ok(AuthenticatedUser {
            user_id: jwt.sub.clone(),
        })
    }
}
