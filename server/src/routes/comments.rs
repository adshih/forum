use super::{AppState, Result};
use crate::auth::{AuthUser, MaybeAuthUser};
use axum::{
    extract::{Path, State},
    routing::{get, post},
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
    id: i64,
    pid: Option<i64>,
    author_id: i64,
    username: String,
    content: String,
    created_at: DateTime<Local>,
    is_voted: bool,
    vote_count: i64,
}

#[derive(Serialize, Deserialize)]
struct VoteCount {
    count: i64,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/threads/:slug/comments",
            post(create_top_level_comment).get(get_comments),
        )
        .route(
            "/api/threads/:slug/comments/:id",
            post(create_nested_comment).get(get_comment),
        )
        .route("/api/threads/:slug/comments/:id/vote", post(vote_comment))
        .route(
            "/api/threads/:slug/comments/:id/unvote",
            post(unvote_comment),
        )
        .route(
            "/api/threads/:slug/comments/:id/children",
            get(get_child_comments),
        )
}

async fn vote_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path((_slug, id)): Path<(String, String)>,
) -> Result<Json<VoteCount>> {
    let id_actual = i64::from_str_radix(&id, 36).unwrap();

    sqlx::query!(
        "
            insert into comment_votes(comment_id, user_id)
            values($1, $2)
            on conflict do nothing
        ",
        id_actual,
        auth_user.id
    )
    .execute(&state.db)
    .await?;

    let count = sqlx::query_as!(
        VoteCount,
        r#"
            select count(*) as "count!"
            from comment_votes
            where comment_id = $1
        "#,
        id_actual
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(count))
}

async fn unvote_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path((_slug, id)): Path<(String, String)>,
) -> Result<Json<VoteCount>> {
    let id_actual = i64::from_str_radix(&id, 36).unwrap();

    sqlx::query!(
        "
            delete from comment_votes
            where comment_id = $1
            and user_id = $2
        ",
        id_actual,
        auth_user.id
    )
    .execute(&state.db)
    .await?;

    let count = sqlx::query_as!(
        VoteCount,
        r#"
            select count(*) as "count!"
            from comment_votes
            where comment_id = $1
        "#,
        id_actual
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(count))
}

async fn create_nested_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path((slug, pid)): Path<(String, String)>,
    Json(req): Json<NewComment>,
) -> Result<Json<Comment>> {
    let id = sqlx::query_scalar!(
        r#"
            insert into comments(thread_id, user_id, content, pid)
            select 
                id as thread_id,
                $2,
                $3,
                $4
            from threads
            where slug = $1
            returning id
        "#,
        slug,
        auth_user.id,
        req.content,
        i64::from_str_radix(&pid, 36).unwrap()
    )
    .fetch_one(&state.db)
    .await?;

    let comment = sqlx::query_as!(
        Comment,
        r#"
            select
                a.id,
                pid,
                user_id as author_id,
                username,
                content,
                a.created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0::bigint as "vote_count!"
            from comments a
            join users b on a.user_id = b.id
            where a.id = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(comment))
}

async fn create_top_level_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<NewComment>,
) -> Result<Json<Comment>> {
    let id = sqlx::query_scalar!(
        r#"
            insert into comments(thread_id, user_id, content)
            select 
                id as thread_id,
                $2,
                $3
            from threads
            where slug = $1
            returning id;
        "#,
        slug,
        auth_user.id,
        req.content
    )
    .fetch_one(&state.db)
    .await?;

    let comment = sqlx::query_as!(
        Comment,
        r#"
            select
                a.id,
                pid,
                user_id as author_id,
                username,
                content,
                a.created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0::bigint as "vote_count!"
            from comments a
            join users b on a.user_id = b.id
            where a.id = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(comment))
}

async fn get_comment(
    auth_user: MaybeAuthUser,
    State(state): State<AppState>,
    Path((slug, id)): Path<(String, String)>,
) -> Result<Json<Comment>> {
    let id = i64::from_str_radix(&id, 36).unwrap();
    let thread_id = sqlx::query_scalar!(
        "
            select id
            from threads
            where slug = $1
        ",
        slug
    )
    .fetch_one(&state.db)
    .await?;

    let comment = sqlx::query_as!(
        Comment,
        r#"
            select
                a.id,
                pid,
                user_id as author_id,
                username,
                content,
                a.created_at as "created_at: DateTime<Local>",
                exists(
                    select *
                    from comment_votes
                    where thread_id = $1
                        and comment_id = a.id
                        and user_id = $3
                ) as "is_voted!",
                (select count(*) from comment_votes where comment_id = a.id) as "vote_count!"
            from comments a
            join users b on a.user_id = b.id
            where a.thread_id = $1
                and a.id = $2
            order by a.created_at desc
        "#,
        thread_id,
        id,
        auth_user.id()
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(comment))
}

async fn get_child_comments(
    State(state): State<AppState>,
    Path((slug, id)): Path<(String, String)>,
) -> Result<Json<Vec<Comment>>> {
    let id = i64::from_str_radix(&id, 36).unwrap();
    let thread_id = sqlx::query_scalar!(
        "
            select id
            from threads
            where slug = $1
        ",
        slug
    )
    .fetch_one(&state.db)
    .await?;

    let comments = sqlx::query_as!(
        Comment,
        r#"
            select
                a.id,
                pid,
                user_id as author_id,
                username,
                content,
                a.created_at as "created_at: DateTime<Local>",
                exists(
                    select *
                    from comment_votes
                    where thread_id = $1
                    and comment_id = a.id
                ) as "is_voted!",
                (select count(*) from comment_votes where comment_id = a.id) as "vote_count!"
            from comments a
            join users b on a.user_id = b.id
            where a.thread_id = $1
                and a.pid = $2
            order by a.created_at desc
        "#,
        thread_id,
        id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}

async fn get_comments(
    auth_user: MaybeAuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<Comment>>> {
    let thread_id = sqlx::query_scalar!(
        "
            select id
            from threads
            where slug = $1
        ",
        slug
    )
    .fetch_one(&state.db)
    .await?;

    let comments = sqlx::query_as!(
        Comment,
        r#"
            select
                a.id,
                pid,
                user_id as author_id,
                username,
                content,
                a.created_at as "created_at: DateTime<Local>",
                exists(
                    select *
                    from comment_votes
                    where thread_id = $1
                        and comment_id = a.id
                        and user_id = $2
                ) as "is_voted!",
                (select count(*) from comment_votes where comment_id = a.id) as "vote_count!"
            from comments a
            join users b on a.user_id = b.id
            where a.thread_id = $1
            order by a.created_at desc
        "#,
        thread_id,
        auth_user.id()
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}
