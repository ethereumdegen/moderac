use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::AppState;
use super::AuthUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct TestRun {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TestResult {
    pub id: String,
    pub run_id: String,
    pub test_id: String,
    pub status: String,
    pub response: Option<String>,
    pub evaluation: Option<String>,
    pub score: Option<f64>,
    pub duration_ms: Option<i32>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct RunDetail {
    pub run: TestRun,
    pub results: Vec<TestResult>,
}

const RUN_SELECT: &str = "SELECT id, project_id, status, started_at::text, completed_at::text, created_at::text FROM test_runs";
const RESULT_SELECT: &str = "SELECT id, run_id, test_id, status, response, evaluation, score, duration_ms, created_at::text FROM test_results";

pub async fn list(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<TestRun>>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let q = format!("{} WHERE project_id = $1 ORDER BY created_at DESC", RUN_SELECT);
    let runs = sqlx::query_as::<_, TestRun>(&q)
        .bind(&project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(runs))
}

pub async fn create(
    State(state): State<AppState>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<TestRun>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let run_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO test_runs (id, project_id, status, started_at) VALUES ($1, $2, 'running', NOW())")
        .bind(&run_id)
        .bind(&project_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tests = sqlx::query_as::<_, (String, String, Option<String>, Option<String>)>(
        "SELECT id, prompt, expected, eval_criteria FROM tests WHERE project_id = $1"
    )
    .bind(&project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut all_passed = true;
    for (test_id, prompt, expected, eval_criteria) in &tests {
        let result_id = Uuid::new_v4().to_string();
        let start = std::time::Instant::now();

        let (status, score, evaluation) = crate::eval::evaluate_test(
            &state.config.openai_api_key,
            prompt,
            expected.as_deref(),
            eval_criteria.as_deref(),
        ).await;

        let duration_ms = start.elapsed().as_millis() as i32;

        if status != "passed" {
            all_passed = false;
        }

        sqlx::query("INSERT INTO test_results (id, run_id, test_id, status, response, evaluation, score, duration_ms) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&result_id)
            .bind(&run_id)
            .bind(test_id)
            .bind(&status)
            .bind(&format!("Evaluated prompt: {}", prompt))
            .bind(&evaluation)
            .bind(score)
            .bind(duration_ms)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let final_status = if all_passed { "passed" } else { "failed" };
    sqlx::query("UPDATE test_runs SET status = $1, completed_at = NOW() WHERE id = $2")
        .bind(final_status)
        .bind(&run_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let q = format!("{} WHERE id = $1", RUN_SELECT);
    let run = sqlx::query_as::<_, TestRun>(&q)
        .bind(&run_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(run))
}

pub async fn get(
    State(state): State<AppState>,
    user: AuthUser,
    Path((project_id, run_id)): Path<(String, String)>,
) -> Result<Json<RunDetail>, StatusCode> {
    verify_project_ownership(&state.db, &project_id, &user.user_id).await?;

    let q = format!("{} WHERE id = $1 AND project_id = $2", RUN_SELECT);
    let run = sqlx::query_as::<_, TestRun>(&q)
        .bind(&run_id)
        .bind(&project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let q2 = format!("{} WHERE run_id = $1", RESULT_SELECT);
    let results = sqlx::query_as::<_, TestResult>(&q2)
        .bind(&run_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RunDetail { run, results }))
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
