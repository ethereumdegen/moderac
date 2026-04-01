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

/// Authenticated user extractor using FutureAuth session cookie.
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

        let session_token = cookie_header
            .split(';')
            .filter_map(|c| {
                let c = c.trim();
                c.strip_prefix("futureauth_session=")
            })
            .next()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Validate session against FutureAuth tables
        let session = sqlx::query_as::<_, (String,)>(
            r#"SELECT user_id FROM session WHERE token = $1 AND expires_at > NOW()"#,
        )
        .bind(session_token)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        let futureauth_user_id = &session.0;

        // Get email from FutureAuth user table
        let fa_user = sqlx::query_as::<_, (Option<String>,)>(
            r#"SELECT email FROM "user" WHERE id = $1"#,
        )
        .bind(futureauth_user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        let email = fa_user.0.unwrap_or_default();

        // Upsert into our local users table
        sqlx::query(
            "INSERT INTO users (id, email) VALUES ($1, $2) ON CONFLICT(email) DO UPDATE SET id = users.id"
        )
        .bind(futureauth_user_id)
        .bind(&email)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Get the actual local user id (may differ if email existed before)
        let (user_id,) = sqlx::query_as::<_, (String,)>(
            "SELECT id FROM users WHERE email = $1"
        )
        .bind(&email)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(AuthUser { user_id, email })
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
