use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;

use crate::AppState;
use super::AuthUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: String,
    pub project_id: String,
    pub key_prefix: String,
    pub name: String,
    pub created_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Serialize)]
pub struct ApiKeyCreated {
    pub id: String,
    pub key: String,
    pub key_prefix: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateApiKey {
    pub name: String,
}

pub async fn list(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<ApiKey>>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let keys = sqlx::query_as::<_, ApiKey>(
        "SELECT id, project_id, key_prefix, name, created_at::text, revoked_at::text FROM api_keys WHERE project_id = $1 ORDER BY created_at DESC"
    )
    .bind(&project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(keys))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
    Json(body): Json<CreateApiKey>,
) -> Result<Json<ApiKeyCreated>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let id = Uuid::new_v4().to_string();
    let raw_key = format!("mdr_{}", hex::encode(rand::random::<[u8; 24]>()));
    let key_prefix = format!("{}...", &raw_key[..12]);
    let key_hash = hex::encode(Sha256::digest(raw_key.as_bytes()));

    sqlx::query("INSERT INTO api_keys (id, project_id, key_hash, key_prefix, name) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&project_id)
        .bind(&key_hash)
        .bind(&key_prefix)
        .bind(&body.name)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKeyCreated {
        id,
        key: raw_key,
        key_prefix,
        name: body.name,
    }))
}

pub async fn revoke(
    State(state): State<AppState>,
    user: AuthUser,
    Path((project_id, key_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let result = sqlx::query("UPDATE api_keys SET revoked_at = NOW() WHERE id = $1 AND project_id = $2")
        .bind(&key_id)
        .bind(&project_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn verify_project_ownership(db: &sqlx::PgPool, project_id: &str, user_id: &str) -> Result<(), StatusCode> {
    sqlx::query_as::<_, (String,)>("SELECT id FROM projects WHERE id = $1 AND user_id = $2")
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(())
}
