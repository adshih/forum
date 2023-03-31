use super::{AppState, Result};
use crate::auth::AuthUser;
use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct NewComment {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Comment {
    pid: Option<i64>,
    author_id: i64,
    username: String,
    content: String,
    created_at: DateTime<Local>,
    is_voted: bool,
    vote_count: i64,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/api/threads/:slug/comments",
        post(create_top_level_comment).get(get_comments),
    )
}

async fn create_top_level_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<NewComment>,
) -> Result<Json<Comment>> {
    sqlx::query!(
        r#"
            insert into comments(thread_id, user_id, content)
            select 
                id as thread_id,
                $2,
                $3
            from threads
            where slug = $1
        "#,
        slug,
        auth_user.id,
        req.content
    )
    .execute(&state.db)
    .await?;

    let comment = sqlx::query_as!(
        Comment,
        r#"
            with x as (
                select t.id thread_id, username
                from threads t
                join users u on t.user_id = u.id
            )

            select
                pid,
                user_id as author_id,
                username,
                content,
                created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0::bigint as "vote_count!"
            from comments c
            join x on c.thread_id = x.thread_id
        "#
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(comment))
}

async fn get_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<Comment>>> {
    let comments = sqlx::query_as!(
        Comment,
        r#"
            with current_thread as (
                select a.id, username
                from threads a
                join users b on a.user_id = b.id
                where slug = $1
            )

            select 
                pid,
                user_id as author_id,
                username,
                content,
                created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0::bigint as "vote_count!"
            from comments
            join current_thread on thread_id = current_thread.id
            order by created_at desc
        "#,
        slug
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}
