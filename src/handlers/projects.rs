use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use super::AuthUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_url: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<Project>>, StatusCode> {
    let projects = sqlx::query_as::<_, Project>("SELECT id, user_id, name, description, base_url, created_at::text FROM projects WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(&user.user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(projects))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<CreateProject>,
) -> Result<Json<Project>, StatusCode> {
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO projects (id, user_id, name, description, base_url) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&user.user_id)
        .bind(&body.name)
        .bind(&body.description)
        .bind(&body.base_url)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project = sqlx::query_as::<_, Project>("SELECT id, user_id, name, description, base_url, created_at::text FROM projects WHERE id = $1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(project))
}

pub async fn get(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    let project = sqlx::query_as::<_, Project>("SELECT id, user_id, name, description, base_url, created_at::text FROM projects WHERE id = $1 AND user_id = $2")
        .bind(&id)
        .bind(&user.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(project))
}

pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(body): Json<UpdateProject>,
) -> Result<Json<Project>, StatusCode> {
    let existing = sqlx::query_as::<_, Project>("SELECT id, user_id, name, description, base_url, created_at::text FROM projects WHERE id = $1 AND user_id = $2")
        .bind(&id)
        .bind(&user.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let name = body.name.unwrap_or(existing.name);
    let description = body.description.or(existing.description);
    let base_url = body.base_url.or(existing.base_url);

    sqlx::query("UPDATE projects SET name = $1, description = $2, base_url = $3 WHERE id = $4")
        .bind(&name)
        .bind(&description)
        .bind(&base_url)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project = sqlx::query_as::<_, Project>("SELECT id, user_id, name, description, base_url, created_at::text FROM projects WHERE id = $1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(project))
}

pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1 AND user_id = $2")
        .bind(&id)
        .bind(&user.user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
