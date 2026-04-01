use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use super::ApiKeyAuth;

#[derive(Deserialize)]
pub struct CreateTestRequest {
    pub name: String,
    pub prompt: String,
    pub expected: Option<String>,
    pub eval_criteria: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct TestResponse {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub status: String,
}

pub async fn create_test(
    State(state): State<AppState>,
    auth: ApiKeyAuth,
    Json(body): Json<CreateTestRequest>,
) -> Result<Json<TestResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let eval_criteria = body.eval_criteria.map(|v| v.to_string());

    sqlx::query("INSERT INTO tests (id, project_id, name, prompt, expected, eval_criteria) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&id)
        .bind(&auth.project_id)
        .bind(&body.name)
        .bind(&body.prompt)
        .bind(&body.expected)
        .bind(&eval_criteria)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TestResponse {
        id,
        name: body.name,
        prompt: body.prompt,
        status: "created".into(),
    }))
}

pub async fn trigger_run(
    State(state): State<AppState>,
    auth: ApiKeyAuth,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let run_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO test_runs (id, project_id, status, started_at) VALUES ($1, $2, 'running', NOW())")
        .bind(&run_id)
        .bind(&auth.project_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tests = sqlx::query_as::<_, (String, String, Option<String>, Option<String>)>(
        "SELECT id, prompt, expected, eval_criteria FROM tests WHERE project_id = $1"
    )
    .bind(&auth.project_id)
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
        if status != "passed" { all_passed = false; }

        sqlx::query("INSERT INTO test_results (id, run_id, test_id, status, response, evaluation, score, duration_ms) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&result_id)
            .bind(&run_id)
            .bind(test_id)
            .bind(&status)
            .bind(&format!("Evaluated: {}", prompt))
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

    Ok(Json(serde_json::json!({
        "run_id": run_id,
        "status": final_status,
        "tests_count": tests.len()
    })))
}

pub async fn get_run(
    State(state): State<AppState>,
    auth: ApiKeyAuth,
    Path(run_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let run = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, String)>(
        "SELECT id, status, started_at::text, completed_at::text, created_at::text FROM test_runs WHERE id = $1 AND project_id = $2"
    )
    .bind(&run_id)
    .bind(&auth.project_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let results = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, Option<f64>, Option<i32>)>(
        "SELECT id, test_id, status, response, evaluation, score, duration_ms FROM test_results WHERE run_id = $1"
    )
    .bind(&run_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let results_json: Vec<serde_json::Value> = results.iter().map(|r| {
        serde_json::json!({
            "id": r.0,
            "test_id": r.1,
            "status": r.2,
            "response": r.3,
            "evaluation": r.4,
            "score": r.5,
            "duration_ms": r.6
        })
    }).collect();

    Ok(Json(serde_json::json!({
        "id": run.0,
        "status": run.1,
        "started_at": run.2,
        "completed_at": run.3,
        "created_at": run.4,
        "results": results_json
    })))
}

#[derive(Deserialize)]
pub struct EvaluateRequest {
    pub prompt: String,
    pub expected: Option<String>,
    pub eval_criteria: Option<serde_json::Value>,
}

pub async fn evaluate(
    State(state): State<AppState>,
    _auth: ApiKeyAuth,
    Json(body): Json<EvaluateRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let (status, score, evaluation) = crate::eval::evaluate_test(
        &state.config.openai_api_key,
        &body.prompt,
        body.expected.as_deref(),
        body.eval_criteria.as_ref().map(|v| v.to_string()).as_deref(),
    ).await;

    Ok(Json(serde_json::json!({
        "status": status,
        "score": score,
        "evaluation": evaluation
    })))
}
