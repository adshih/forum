use super::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct Profile {
    username: String,
    created_at: String,
    // is_following: bool,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/profiles/:username", get(get_profile))
}

#[debug_handler]
async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, StatusCode> {
    let result = sqlx::query!(
        r#"select username, created_at from users where username = $1"#,
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let local_created_at: DateTime<Local> = DateTime::from(result.created_at.unwrap());

    Ok(Json(Profile {
        username: result.username,
        created_at: local_created_at.to_string(),
    }))
}
