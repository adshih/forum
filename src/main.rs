use axum::{routing::get, Router};
use forum::routes::{self, AppState};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Using this to get environment variables during development.
    // Will need to remove this when deployed to production.
    dotenvy::dotenv().ok();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .expect("cound not connect to database");

    let app_state = AppState { db };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::users::router())
        .merge(routes::profiles::router())
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
