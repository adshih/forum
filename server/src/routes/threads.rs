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
struct Thread {
    title: String,
    slug: String,
    content: String,
    created_at: DateTime<Local>,
    is_voted: bool,
    vote_count: i64,
    author_id: i64,
}

#[derive(Serialize)]
struct Listing {
    threads: Vec<Thread>,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/threads", post(create_thread).get(get_listing))
        .route("/api/threads/:slug", get(get_thread))
    // .route(
    //     "/api/threads/:slug/vote",
    //     post(vote_thread).delete(unvote_thread),
    // )
}

async fn _vote_thread(
    _auth_user: AuthUser,
    State(_state): State<AppState>,
    Path(_slug): Path<String>,
) {
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
                title, 
                slug, 
                content, 
                created_at as "created_at: DateTime<Local>",
                exists(select * from votes where user_id = $1) as "is_voted!",
                (select count(*) from votes where thread_id = threads.id) as "vote_count!"
            from threads
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
                title, 
                slug, 
                content, 
                created_at as "created_at: DateTime<Local>",
                exists(select * from votes where user_id = $1) as "is_voted!",
                (select count(*) from votes where thread_id = threads.id) as "vote_count!"
            from threads 
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

    let thread = sqlx::query_as!(
        Thread,
        r#"
            insert into threads(user_id, title, slug, content)
            values($1, $2, $3, $4)
            returning
                user_id as author_id,
                slug,
                title,
                content,
                created_at as "created_at: DateTime<Local>",
                0 as "vote_count!: i64",
                false as "is_voted!"
        "#,
        auth_user.id,
        req.title,
        slug,
        req.content
    )
    .fetch_one(&state.db)
    .await
    .on_constraint("threads_slug_key", |_| {
        Error::unprocessable_entity([("slug", format!("duplicate thread slug: {}", slug))])
    })?;

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
