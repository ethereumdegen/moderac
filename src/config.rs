use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub openai_api_key: String,
    pub listen_addr: String,
    pub futureauth_api_url: String,
    pub futureauth_secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            openai_api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| {
                let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
                format!("0.0.0.0:{}", port)
            }),
            futureauth_api_url: env::var("FUTUREAUTH_API_URL")
                .unwrap_or_else(|_| "https://future-auth.com".into()),
            futureauth_secret_key: env::var("FUTUREAUTH_SECRET_KEY")
                .expect("FUTUREAUTH_SECRET_KEY is required"),
        }
    }
}
