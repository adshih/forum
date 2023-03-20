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
    author_id: i64,
    content: String,
    created_at: DateTime<Local>,
    is_voted: bool,
    vote_count: i32,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/api/comments/:slug",
        post(create_top_level_comment).get(get_comments),
    )
}

async fn create_top_level_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<NewComment>,
) -> Result<Json<Comment>> {
    let comment = sqlx::query_as!(
        Comment,
        r#"
            insert into comments(thread_id, user_id, content)
            select 
                id as thread_id,
                $2,
                $3
            from threads
            where slug = $1
            returning
                user_id as author_id,
                content,
                created_at as "created_at: DateTime<Local>",
                false as "is_voted!",
                0 as "vote_count!"
        "#,
        slug,
        auth_user.id,
        req.content
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(comment))
}

async fn get_comments() {}
