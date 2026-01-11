use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use clerk_rs::validators::authorizer::ClerkJwt;
use serde::Deserialize;
use std::sync::OnceLock;

pub use clerk_rs::{
    ClerkConfiguration,
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
};

static CLERK_CLIENT: OnceLock<Clerk> = OnceLock::new();
static CLERK_SECRET_KEY: OnceLock<String> = OnceLock::new();

pub fn init_clerk(secret_key: String) {
    CLERK_SECRET_KEY.set(secret_key.clone()).ok();
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let clerk = Clerk::new(config);
    CLERK_CLIENT.set(clerk).ok();
}

pub fn get_clerk() -> Option<&'static Clerk> {
    CLERK_CLIENT.get()
}

#[derive(Debug, Deserialize)]
struct ClerkUser {
    id: String,
    email_addresses: Option<Vec<ClerkEmail>>,
}

#[derive(Debug, Deserialize)]
struct ClerkEmail {
    email_address: String,
}

// Workaround: clerk-rs crate doesn't provide a method to query users by email,
// so we use the Clerk REST API directly via HTTP request.
// TODO: Switch to clerk-rs API when/if this functionality is added to the crate.
pub async fn get_user_id_by_email(email: &str) -> Result<Option<String>, String> {
    let secret_key = CLERK_SECRET_KEY.get().ok_or("Clerk not initialized")?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://api.clerk.com/v1/users?email_address={}",
            urlencoding::encode(email)
        ))
        .header("Authorization", format!("Bearer {}", secret_key))
        .send()
        .await
        .map_err(|e| format!("Failed to query Clerk API: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Clerk API returned status: {}", response.status()));
    }

    let users: Vec<ClerkUser> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Clerk response: {}", e))?;

    Ok(users.first().map(|u| u.id.clone()))
}

pub async fn get_user_email_by_id(user_id: &str) -> Result<Option<String>, String> {
    let secret_key = CLERK_SECRET_KEY.get().ok_or("Clerk not initialized")?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.clerk.com/v1/users/{}", user_id))
        .header("Authorization", format!("Bearer {}", secret_key))
        .send()
        .await
        .map_err(|e| format!("Failed to query Clerk API: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Clerk API returned status: {}", response.status()));
    }

    let user: ClerkUser = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Clerk response: {}", e))?;

    Ok(user
        .email_addresses
        .and_then(|emails| emails.first().map(|e| e.email_address.clone())))
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
