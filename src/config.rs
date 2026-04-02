use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: Option<String>,
    pub openai_api_key: Option<String>,
    pub listen_addr: String,
    pub futureauth_api_url: String,
    pub futureauth_secret_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").ok(),
            openai_api_key: env::var("OPENAI_API_KEY").ok().filter(|s| !s.is_empty()),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| {
                let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
                format!("0.0.0.0:{}", port)
            }),
            futureauth_api_url: env::var("FUTUREAUTH_API_URL")
                .unwrap_or_else(|_| "https://future-auth.com".into()),
            futureauth_secret_key: env::var("FUTUREAUTH_SECRET_KEY").ok(),
        }
    }

    pub fn has_db(&self) -> bool {
        self.database_url.is_some()
    }

    pub fn has_auth(&self) -> bool {
        self.database_url.is_some() && self.futureauth_secret_key.is_some()
    }

    pub fn has_eval(&self) -> bool {
        self.openai_api_key.is_some()
    }
}
