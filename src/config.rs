use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub openai_api_key: String,
    pub listen_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| {
                // Railway sets PORT
                let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
                format!("0.0.0.0:{}", port)
            }),
        }
    }
}
