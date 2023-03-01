use axum::{routing::get, Router};
use forum::http::{self, ApiContext};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Using this to get environment variables during development.
    // Will need to remove this when deployed to production.
    dotenv::dotenv().ok();

    let db_url = dotenv::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .expect("cound not connect to database");

    let api_context = ApiContext { db };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(http::users::router())
        .with_state(api_context);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
