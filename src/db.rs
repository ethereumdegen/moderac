use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;

pub async fn init_pool(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    pool.execute(include_str!("../migrations/001_init.sql"))
        .await
        .expect("Failed to run migrations");

    pool
}
