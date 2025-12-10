use axum::Router;
use dotenvy::dotenv;

mod db;
mod data;
mod metas;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db().await.expect("Failed to connect to database");

    let app = Router::new().nest("/arena", metas::arena::router()).with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
