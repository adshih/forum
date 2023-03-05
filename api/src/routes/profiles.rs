use crate::auth::AuthUser;

use super::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Profile {
    username: String,
    // score: i32,
    // created_at: String,
    // is_following: bool,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/profiles/:username", get(get_profile))
        .route(
            "/api/profiles/:username/follow",
            post(follow_user).delete(unfollow_user),
        )
}

async fn follow_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, StatusCode> {
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query!(
        "select id, username from users where username = $1",
        username
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    sqlx::query!(
        "insert into follows(followee_user_id, follower_user_id) values($1, $2) on conflict do nothing",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Profile {
        username: user.username,
    }))
}

async fn unfollow_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, StatusCode> {
    let mut tx = state
        .db
        .begin()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query!(
        "select id, username from users where username = $1",
        username
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    sqlx::query!(
        "delete from follows where followee_user_id = $1 and follower_user_id = $2",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Profile {
        username: user.username,
    }))
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, StatusCode> {
    let result = sqlx::query!(
        r#"select username, score, created_at from users where username = $1"#,
        username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let _local_created_at: DateTime<Local> = DateTime::from(result.created_at.unwrap());

    Ok(Json(Profile {
        username: result.username,
        // score: result.score.unwrap(),
        // created_at: local_created_at.to_string(),
    }))
}
