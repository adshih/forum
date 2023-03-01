use sqlx::PgPool;

pub mod users;

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}
