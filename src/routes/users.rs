use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;

use super::AppState;

#[derive(Deserialize, Debug)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    username: String,
    email: String,
    token: String,
    created_at: String,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/protected", get(protected_handler))
        .route("/api/users", post(create_user).get(get_user))
        .route("/api/users/login", post(login_user))
    // get put "/api/users"
    // post delete "/api/users/login"
}

async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    println!("Logged in as {}", user.username);
}

async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query!("select * from users where username = $1", req.username)
        .fetch_one(&state.db)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    verify_password(req.password, user.password.clone()).await?;

    Ok(Json(User {
        email: user.email,
        token: AuthUser { user_id: user.id }.to_jwt(&state),
        username: user.username,
        created_at: DateTime::<Local>::from(user.created_at.unwrap()).to_string(),
    }))
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
        username: req.username,
        email: req.email,
        token: AuthUser { user_id: result.id }.to_jwt(&state),
        created_at: local_created_at.to_string(),
    }))
}

async fn get_user(auth_user: AuthUser) -> String {
    auth_user.user_id.to_string()
}

async fn hash_password(password: String) -> Result<String, StatusCode> {
    let salt = SaltString::generate(rand::thread_rng());

    PasswordHash::generate(Argon2::default(), password, salt.as_str())
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| v.to_string())
}

async fn verify_password(password: String, password_hash: String) -> Result<(), StatusCode> {
    let hash = PasswordHash::new(&password_hash).map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    hash.verify_password(&[&Argon2::default()], password)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
