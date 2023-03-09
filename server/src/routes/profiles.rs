use super::AppState;
use super::Result;
use crate::auth::AuthUser;
use crate::error::{Error, ResultExt};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Profile {
    username: String,
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
) -> Result<Json<Profile>> {
    let mut tx = state.db.begin().await?;

    let user = sqlx::query!(
        "select id, username from users where username = $1",
        username
    )
    .fetch_optional(&mut tx)
    .await?
    .ok_or(Error::NotFound)?;

    sqlx::query!(
        "insert into follows(followee_user_id, follower_user_id) values($1, $2) on conflict do nothing",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await
    .on_constraint("user_cannot_follow_self", |_| Error::Forbidden)?;

    tx.commit().await?;

    Ok(Json(Profile {
        username: user.username,
    }))
}

async fn unfollow_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    let mut tx = state.db.begin().await?;

    let user = sqlx::query!(
        "select id, username from users where username = $1",
        username
    )
    .fetch_optional(&mut tx)
    .await?
    .ok_or(Error::NotFound)?;

    sqlx::query!(
        "delete from follows where followee_user_id = $1 and follower_user_id = $2",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(Json(Profile {
        username: user.username,
    }))
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    let result = sqlx::query!(
        r#"select username, score, created_at from users where username = $1"#,
        username
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(Profile {
        username: result.username,
    }))
}
