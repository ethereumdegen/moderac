use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use super::AuthUser;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let code = format!("{:06}", rand::random::<u32>() % 1_000_000);
    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO auth_codes (id, email, code, expires_at) VALUES ($1, $2, $3, NOW() + INTERVAL '10 minutes')")
        .bind(&id)
        .bind(&body.email)
        .bind(&code)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("Auth code for {}: {}", body.email, code);

    Ok(Json(LoginResponse {
        message: "Code sent to your email".into(),
    }))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub email: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
}

pub async fn verify(
    State(state): State<AppState>,
    Json(body): Json<VerifyRequest>,
) -> Result<(StatusCode, [(String, String); 1], Json<VerifyResponse>), StatusCode> {
    let row = sqlx::query_as::<_, (String,)>(
        "SELECT id FROM auth_codes WHERE email = $1 AND code = $2 AND used = FALSE AND expires_at > NOW() ORDER BY created_at DESC LIMIT 1"
    )
    .bind(&body.email)
    .bind(&body.code)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    sqlx::query("UPDATE auth_codes SET used = TRUE WHERE id = $1")
        .bind(&row.0)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2) ON CONFLICT(email) DO NOTHING")
        .bind(&user_id)
        .bind(&body.email)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (actual_user_id, email) = sqlx::query_as::<_, (String, String)>(
        "SELECT id, email FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES ($1, $2, NOW() + INTERVAL '30 days')")
        .bind(&session_id)
        .bind(&actual_user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = format!("session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000", session_id);

    Ok((
        StatusCode::OK,
        [("set-cookie".into(), cookie)],
        Json(VerifyResponse {
            user: UserInfo {
                id: actual_user_id,
                email,
            },
        }),
    ))
}

pub async fn me(user: AuthUser) -> Json<UserInfo> {
    Json(UserInfo {
        id: user.user_id,
        email: user.email,
    })
}

pub async fn logout(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<(StatusCode, [(String, String); 1]), StatusCode> {
    sqlx::query("DELETE FROM sessions WHERE user_id = $1")
        .bind(&user.user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = "session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0";
    Ok((StatusCode::OK, [("set-cookie".into(), cookie.into())]))
}
