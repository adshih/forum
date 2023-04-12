use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

mod comments;
mod profiles;
mod threads;
pub mod users;

#[derive(Clone)]
pub(crate) struct AppState {
    pub db: PgPool,
    pub key: String,
}

pub use crate::error::{Error, ResultExt};
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn router(db: PgPool) -> Router {
    let app_state = AppState {
        db,
        key: "secret_idk".to_string(),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("https://54.185.58.189".parse::<HeaderValue>().unwrap());

    Router::new()
        .route("/", get(root_handler))
        .merge(users::router())
        .merge(profiles::router())
        .merge(threads::router())
        .merge(comments::router())
        .layer(cors)
        .with_state(app_state)
}

async fn root_handler() -> Json<String> {
    Json("Welcome to my forum!".to_string())
}
