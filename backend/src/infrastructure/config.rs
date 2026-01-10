pub struct AppConfig {
    pub port: u16,
    pub openai_api_key: String,
    pub cors_origin: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            openai_api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        }
    }
}
