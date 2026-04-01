use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use super::AuthUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct Test {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub prompt: String,
    pub expected: Option<String>,
    pub eval_criteria: Option<String>,
    pub config: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateTest {
    pub name: String,
    pub prompt: String,
    pub expected: Option<String>,
    pub eval_criteria: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateTest {
    pub name: Option<String>,
    pub prompt: Option<String>,
    pub expected: Option<String>,
    pub eval_criteria: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
}

const TEST_SELECT: &str = "SELECT id, project_id, name, prompt, expected, eval_criteria, config, created_at::text, updated_at::text FROM tests";

pub async fn list(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<Test>>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let q = format!("{} WHERE project_id = $1 ORDER BY created_at DESC", TEST_SELECT);
    let tests = sqlx::query_as::<_, Test>(&q)
        .bind(&project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tests))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
    Json(body): Json<CreateTest>,
) -> Result<Json<Test>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let id = Uuid::new_v4().to_string();
    let eval_criteria = body.eval_criteria.map(|v| v.to_string());
    let config = body.config.map(|v| v.to_string());

    sqlx::query("INSERT INTO tests (id, project_id, name, prompt, expected, eval_criteria, config) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&id)
        .bind(&project_id)
        .bind(&body.name)
        .bind(&body.prompt)
        .bind(&body.expected)
        .bind(&eval_criteria)
        .bind(&config)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let q = format!("{} WHERE id = $1", TEST_SELECT);
    let test = sqlx::query_as::<_, Test>(&q)
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(test))
}

pub async fn get(
    State(state): State<AppState>,
    user: AuthUser,
    Path((project_id, test_id)): Path<(String, String)>,
) -> Result<Json<Test>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let q = format!("{} WHERE id = $1 AND project_id = $2", TEST_SELECT);
    let test = sqlx::query_as::<_, Test>(&q)
        .bind(&test_id)
        .bind(&project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(test))
}

pub async fn update(
    State(state): State<AppState>,
    user: AuthUser,
    Path((project_id, test_id)): Path<(String, String)>,
    Json(body): Json<UpdateTest>,
) -> Result<Json<Test>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let q = format!("{} WHERE id = $1 AND project_id = $2", TEST_SELECT);
    let existing = sqlx::query_as::<_, Test>(&q)
        .bind(&test_id)
        .bind(&project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let name = body.name.unwrap_or(existing.name);
    let prompt = body.prompt.unwrap_or(existing.prompt);
    let expected = body.expected.or(existing.expected);
    let eval_criteria = body.eval_criteria.map(|v| v.to_string()).or(existing.eval_criteria);
    let config = body.config.map(|v| v.to_string()).or(existing.config);

    sqlx::query("UPDATE tests SET name = $1, prompt = $2, expected = $3, eval_criteria = $4, config = $5, updated_at = NOW() WHERE id = $6")
        .bind(&name)
        .bind(&prompt)
        .bind(&expected)
        .bind(&eval_criteria)
        .bind(&config)
        .bind(&test_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let q2 = format!("{} WHERE id = $1", TEST_SELECT);
    let test = sqlx::query_as::<_, Test>(&q2)
        .bind(&test_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(test))
}

pub async fn delete(
    State(state): State<AppState>,
    user: AuthUser,
    Path((project_id, test_id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let result = sqlx::query("DELETE FROM tests WHERE id = $1 AND project_id = $2")
        .bind(&test_id)
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
