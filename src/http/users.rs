use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use super::ApiContext;

#[derive(Deserialize, Debug)]
struct NewUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
}

pub fn router() -> Router<ApiContext> {
    Router::new().route("/api/users", post(create_user))
}

#[debug_handler]
async fn create_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let password_hash = hash_password(req.password).await?;

    let _user_id = sqlx::query_scalar!(
        r#"insert into users (username, password) values ($1, $2) returning id"#,
        req.username,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(User {
        username: req.username,
    }))
}

async fn hash_password(password: String) -> Result<String, StatusCode> {
    let salt = SaltString::generate(rand::thread_rng());

    PasswordHash::generate(Argon2::default(), password, salt.as_str())
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| v.to_string())
}
