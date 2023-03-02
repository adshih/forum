use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::AppState;

#[derive(Deserialize, Debug)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    token: String,
    username: String,
    email: String,
    created_at: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/users", post(create_user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let password_hash = hash_password(req.password).await?;

    let result = sqlx::query!(
        r#"insert into users (username, email, password) values ($1, $2, $3) returning id, created_at"#,
        req.username,
        req.email,
        password_hash,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let local_created_at: DateTime<Local> = DateTime::from(result.created_at.unwrap());

    Ok(Json(User {
        id: result.id.to_string(),
        token: "12341234".to_string(),
        username: req.username,
        email: req.email,
        created_at: local_created_at.to_string(),
    }))
}

async fn hash_password(password: String) -> Result<String, StatusCode> {
    let salt = SaltString::generate(rand::thread_rng());

    PasswordHash::generate(Argon2::default(), password, salt.as_str())
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| v.to_string())
}

// Will implement login after I decide on what method of authorization to use.

// #[derive(Deserialize)]
// struct LoginUser {
//     email: String,
//     password: String,
// }

// async fn _login_user(
//     State(state): State<AppState>,
//     Json(req): Json<LoginUser>,
// ) -> Result<Json<LoginUser>, StatusCode> {
//     Ok(Json(LoginUser {
//         email: "".to_string(),
//         password: "".to_string(),
//     }))
// }

// async fn _verify_password(password: String, hash: String) -> Result<(), StatusCode> {
//     Ok(())
// }
