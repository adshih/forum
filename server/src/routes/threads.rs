use super::{AppState, Error, Result, ResultExt};
use crate::auth::AuthUser;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Local};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct NewThread {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub author_id: i64,
    pub username: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub is_voted: bool,
    pub vote_count: i64,
}

#[derive(Serialize)]
struct Listing {
    threads: Vec<Thread>,
}

#[derive(Serialize)]
struct VoteCount {
    count: i64,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/threads", post(create_thread).get(get_listing))
        .route("/api/threads/:slug", get(get_thread))
        .route("/api/threads/:slug/vote", post(vote).get(get_votes))
}

async fn get_votes(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<VoteCount>> {
    let count = sqlx::query_as!(
        VoteCount,
        r#"
            with selected_thread as (
                select id
                from threads
                where slug = $1
            )
            
            select count(*) as "count!"
            from thread_votes
            join selected_thread on thread_id = id
        "#,
        slug
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(count))
}

async fn vote(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<VoteCount>> {
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

    sqlx::query!(
        "
            insert into thread_votes(thread_id, user_id)
            values($1, $2)
            on conflict do nothing
        ",
        thread_id,
        auth_user.id
    )
    .execute(&state.db)
    .await?;

    let count = sqlx::query_as!(
        VoteCount,
        r#"
            select count(*) as "count!"
            from thread_votes
            where thread_id = $1
        "#,
        thread_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(count))
}

async fn _unvote_thread(
    _auth_user: AuthUser,
    State(_state): State<AppState>,
    Path(_slug): Path<String>,
) {
}

async fn get_listing(
    auth_user: Option<AuthUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Thread>>> {
    let user_id = if let Some(user) = auth_user {
        Some(user.id)
    } else {
        None
    };

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
                exists(select * from thread_votes where user_id = $1) as "is_voted!",
                (select count(*) from thread_votes where thread_id = a.id) as "vote_count!"
            from threads a
            join users b on a.user_id = b.id
            order by a.created_at desc
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(threads))
}

async fn get_thread(
    auth_user: Option<AuthUser>,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Thread>> {
    let user_id = if let Some(user) = auth_user {
        Some(user.id)
    } else {
        None
    };

    let thread = sqlx::query_as!(
        Thread,
        r#"
            select
                user_id as author_id,
                username,
                slug, 
                title, 
                content, 
                a.created_at as "created_at: DateTime<Local>",
                exists(select * from thread_votes where user_id = $1) as "is_voted!",
                (select count(*) from thread_votes where thread_id = a.id) as "vote_count!"
            from threads a
            join users b on a.user_id = b.id
            where slug = $2
        "#,
        user_id,
        slug
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(thread))
}

async fn create_thread(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<NewThread>,
) -> Result<Json<Thread>> {
    let slug = slugify(&req.title);

    sqlx::query!(
        r#"
            insert into threads(user_id, title, slug, content)
            values($1, $2, $3, $4)
        "#,
        auth_user.id,
        req.title,
        slug,
        req.content
    )
    .execute(&state.db)
    .await
    .on_constraint("threads_slug_key", |_| {
        Error::unprocessable_entity([("slug", format!("duplicate thread slug: {}", slug))])
    })?;

    let thread = sqlx::query_as!(
        Thread,
        r#"
            select
                user_id as author_id,
                username,
                slug, 
                title, 
                content, 
                a.created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0::bigint as "vote_count!" 
            from threads a
            join users b on a.user_id = b.id
            where slug = $1
        "#,
        slug
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(thread))
}

fn slugify(title: &str) -> String {
    let quotes = ['\'', '\"'];

    title
        .split(|c| !(quotes.contains(&c) || c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.replace(quotes, "");
            s.make_ascii_lowercase();
            s
        })
        .join("-")
}
