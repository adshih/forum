use super::threads::Thread;
use super::AppState;
use super::Result;
use crate::auth::AuthUser;
use crate::error::{Error, ResultExt};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Profile {
    username: String,
    score: i64,
    created_at: DateTime<Local>,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/profiles/:username", get(get_profile))
        .route(
            "/api/profiles/:username/follow",
            post(follow_user).delete(unfollow_user),
        )
        .route("/api/profiles/:username/threads", get(get_threads))
}

async fn get_threads(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Thread>>> {
    let threads = sqlx::query_as!(
        Thread,
        r#"
            select
                user_id as author_id,
                b.username,
                slug,
                title,
                content,
                a.created_at as "created_at: DateTime<Local>",
                exists(
                    select * 
                    from thread_votes 
                    where user_id = b.id 
                        and thread_id = a.id
                ) as "is_voted!",
                (select count(*) from thread_votes where thread_id = a.id) as "vote_count!"
            from threads a
            join users b on a.user_id = b.id
            where b.username = $1
            order by a.created_at desc
        "#,
        username
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(threads))
}

async fn follow_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    let mut tx = state.db.begin().await?;

    let user = sqlx::query!(
        r#"
            select 
                id, 
                username,
                created_at as "created_at: DateTime<Local>"
            from users 
            where username = $1
        "#,
        username
    )
    .fetch_optional(&mut tx)
    .await?
    .ok_or(Error::NotFound)?;

    let score = sqlx::query!(
        r#"
            select count(*) as "count!"
            from thread_votes
            where user_id = $1
        "#,
        user.id
    )
    .fetch_one(&mut tx)
    .await?;

    sqlx::query!(
        "
            insert into follows(followee_user_id, follower_user_id) 
            values($1, $2) 
            on conflict do nothing
        ",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await
    .on_constraint("user_cannot_follow_self", |_| Error::Forbidden)?;

    tx.commit().await?;

    Ok(Json(Profile {
        username: user.username,
        score: score.count,
        created_at: user.created_at,
    }))
}

async fn unfollow_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    let mut tx = state.db.begin().await?;

    let user = sqlx::query!(
        r#"
            select 
                id, 
                username,
                created_at as "created_at: DateTime<Local>"
            from users 
            where username = $1
        "#,
        username
    )
    .fetch_optional(&mut tx)
    .await?
    .ok_or(Error::NotFound)?;

    let score = sqlx::query!(
        r#"
            select count(*) as "count!"
            from thread_votes
            where user_id = $1
        "#,
        user.id
    )
    .fetch_one(&mut tx)
    .await?;

    sqlx::query!(
        "
            delete from follows 
            where followee_user_id = $1 and follower_user_id = $2
        ",
        user.id,
        auth_user.id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(Json(Profile {
        username: user.username,
        score: score.count,
        created_at: user.created_at,
    }))
}

async fn get_profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    let user = sqlx::query!(
        r#"
            select 
                id, 
                username,
                created_at as "created_at: DateTime<Local>"
            from users 
            where username = $1
        "#,
        username
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NotFound)?;

    let score = sqlx::query!(
        r#"
            select count(*) as "count!"
            from thread_votes
            where user_id = $1
        "#,
        user.id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(Profile {
        username: user.username,
        score: score.count,
        created_at: user.created_at,
    }))
}
