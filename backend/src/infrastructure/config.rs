pub struct AppConfig {
    pub port: u16,
    pub openai_api_key: String,
    pub cors_origin: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());

        let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);

        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            openai_api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            database_url,
        }
    }
}
