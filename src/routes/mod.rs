use sqlx::PgPool;

pub mod profiles;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
