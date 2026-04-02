mod config;
mod db;
mod eval;
mod handlers;

use axum::{
    routing::{get, post, delete},
    Router, Json,
};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{CorsLayer, Any};
use futureauth::{FutureAuth, FutureAuthConfig};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: config::Config,
    pub auth: Arc<FutureAuth>,
}

impl AsRef<Arc<FutureAuth>> for AppState {
    fn as_ref(&self) -> &Arc<FutureAuth> {
        &self.auth
    }
}

#[derive(Serialize)]
struct Features {
    auth: bool,
    eval: bool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let listen_addr = config.listen_addr.clone();
    let spa_fallback = ServeFile::new("frontend/dist/index.html");

    let app = if let (Some(database_url), Some(secret_key)) =
        (config.database_url.clone(), config.futureauth_secret_key.clone())
    {
        // Full mode: database + auth + API routes
        let pool = db::init_pool(&database_url).await;

        let auth = FutureAuth::new(pool.clone(), FutureAuthConfig {
            api_url: config.futureauth_api_url.clone(),
            secret_key,
            project_name: "Moderac".to_string(),
            ..Default::default()
        });
        auth.ensure_tables().await.expect("Failed to create FutureAuth tables");

        let state = AppState {
            db: pool,
            config,
            auth,
        };

        let features = {
            let s = state.clone();
            move || async move {
                Json(Features {
                    auth: s.config.has_auth(),
                    eval: s.config.has_eval(),
                })
            }
        };

        let api = Router::new()
            // FutureAuth routes (send-otp, verify-otp, session, sign-out)
            .merge(futureauth::axum::auth_router(state.auth.clone()))
            // Features
            .route("/features", get(features))
            // Projects
            .route("/projects", get(handlers::projects::list).post(handlers::projects::create))
            .route("/projects/{id}", get(handlers::projects::get).put(handlers::projects::update).delete(handlers::projects::delete))
            // Tests
            .route("/projects/{project_id}/tests", get(handlers::tests::list).post(handlers::tests::create))
            .route("/projects/{project_id}/tests/{test_id}", get(handlers::tests::get).put(handlers::tests::update).delete(handlers::tests::delete))
            // Runs
            .route("/projects/{project_id}/runs", get(handlers::runs::list).post(handlers::runs::create))
            .route("/projects/{project_id}/runs/{run_id}", get(handlers::runs::get))
            // API Keys
            .route("/projects/{project_id}/api-keys", get(handlers::api_keys::list).post(handlers::api_keys::create))
            .route("/projects/{project_id}/api-keys/{key_id}", delete(handlers::api_keys::revoke))
            // Public API (API key auth)
            .route("/v1/tests", post(handlers::public_api::create_test))
            .route("/v1/runs", post(handlers::public_api::trigger_run))
            .route("/v1/runs/{run_id}", get(handlers::public_api::get_run))
            .route("/v1/evaluate", post(handlers::public_api::evaluate));

        Router::new()
            .nest("/api", api)
            .fallback_service(
                ServeDir::new("frontend/dist").fallback(spa_fallback)
            )
            .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
            .with_state(state)
    } else {
        // Bare landing page mode: no database, no auth, just static files + features endpoint
        tracing::warn!("DATABASE_URL or FUTUREAUTH_SECRET_KEY not set — running in landing-page-only mode");

        let api = Router::new()
            .route("/features", get(|| async {
                Json(Features { auth: false, eval: false })
            }));

        Router::new()
            .nest("/api", api)
            .fallback_service(
                ServeDir::new("frontend/dist").fallback(spa_fallback)
            )
            .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
    };

    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
    tracing::info!("Listening on {}", listen_addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
