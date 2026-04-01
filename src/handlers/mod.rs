pub mod auth;
pub mod projects;
pub mod tests;
pub mod runs;
pub mod api_keys;
pub mod public_api;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::AppState;

/// Authenticated user extractor
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_id = cookie_header
            .split(';')
            .filter_map(|c| {
                let c = c.trim();
                c.strip_prefix("session=")
            })
            .next()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let row = sqlx::query_as::<_, (String, String)>(
            "SELECT u.id, u.email FROM sessions s JOIN users u ON s.user_id = u.id WHERE s.id = $1 AND s.expires_at > NOW()"
        )
        .bind(session_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            user_id: row.0,
            email: row.1,
        })
    }
}

/// API key auth extractor for public API
#[derive(Debug, Clone)]
pub struct ApiKeyAuth {
    pub project_id: String,
}

impl FromRequestParts<AppState> for ApiKeyAuth {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let key_hash = hex::encode(sha2::Digest::finalize(sha2::Sha256::new().chain_update(auth_header.as_bytes())));

        let row = sqlx::query_as::<_, (String,)>(
            "SELECT project_id FROM api_keys WHERE key_hash = $1 AND revoked_at IS NULL"
        )
        .bind(&key_hash)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(ApiKeyAuth {
            project_id: row.0,
        })
    }
}

use sha2::Digest;
