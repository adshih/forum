use super::{AppState, Error, Result, ResultExt};
use crate::auth::AuthUser;
use axum::{extract::State, routing::post, Json, Router};
use axum_macros::debug_handler;
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
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/api/threads", post(create_thread))
    // .route("/api/threads/:slug", get(get_thread))
    // .route("/api/threads/vote", post(vote_thread).delete(unvote_thread))
}

#[debug_handler]
async fn create_thread(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<NewThread>,
) -> Result<Json<Thread>> {
    let slug = slugify(&req.title);

    let _result = sqlx::query!(
        "insert into threads(user_id, title, slug, content) values($1, $2, $3, $4)",
        auth_user.id,
        req.title,
        slug,
        req.content
    )
    .execute(&state.db)
    .await
    .on_constraint("article_slug_key", |_| {
        Error::unprocessable_entity([("slug", format!("duplicate article slug: {}", slug))])
    })?;

    Ok(Json(Thread {
        title: req.title,
        slug: slug,
        content: req.content,
    }))
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
