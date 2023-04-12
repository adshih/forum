use super::{AppState, Error, Result, ResultExt};
use crate::auth::AuthUser;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{extract::State, routing::post, Json, Router};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct NewUser {
    username: String,
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
    // email: String,
    token: String,
    created_at: DateTime<Local>,
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", post(create_user).get(get_user))
        .route("/api/users/login", post(login_user))
}

async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginUser>,
) -> Result<Json<User>> {
    let user = sqlx::query!(
        "
            select 
                id, 
                username, 
                password_hash, 
                created_at 
            from users where username = $1
        ",
        req.username
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| Error::unprocessable_entity([("username", "does not exist")]))?;

    verify_password(req.password, user.password_hash.clone()).await?;

    Ok(Json(User {
        token: AuthUser { id: user.id }.to_jwt(&state),
        username: user.username,
        created_at: user.created_at.into(),
    }))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<NewUser>,
) -> Result<Json<User>> {
    let password_hash = hash_password(req.password).await?;

    let result = sqlx::query!(
        "
            insert into users (username, password_hash) 
            values ($1, $2) 
            returning id, created_at
        ",
        req.username,
        password_hash,
    )
    .fetch_one(&state.db)
    .await
    .on_constraint("users_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })?;

    Ok(Json(User {
        username: req.username,
        token: AuthUser { id: result.id }.to_jwt(&state),
        created_at: result.created_at.into(),
    }))
}

async fn get_user(auth_user: AuthUser, State(state): State<AppState>) -> Result<Json<User>> {
    let user = sqlx::query!(
        r#"
            select
                username,
                created_at "created_at: DateTime<Local>"
            from users
            where id = $1
        "#,
        auth_user.id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(User {
        username: user.username,
        token: auth_user.to_jwt(&state),
        created_at: user.created_at,
    }))
}

async fn hash_password(password: String) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());

    Ok(
        PasswordHash::generate(Argon2::default(), password, salt.as_str())
            .map_err(|e| anyhow::anyhow!("Failed to generate password hash: {}", e))?
            .to_string(),
    )
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    let hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

    hash.verify_password(&[&Argon2::default()], password)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => Error::Unauthorized,
            _ => anyhow::anyhow!("Failed to verify password hash: {}", e).into(),
        })
}
