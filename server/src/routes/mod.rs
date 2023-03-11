use axum::{routing::get, Json, Router};
use sqlx::PgPool;

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

    Router::new()
        .route("/", get(root_handler))
        .merge(users::router())
        .merge(profiles::router())
        .merge(threads::router())
        .with_state(app_state)
}

async fn root_handler() -> Json<String> {
    Json("Aw Rats...".to_string())
}
