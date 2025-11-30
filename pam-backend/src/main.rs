use axum::Router;

mod data;
mod metas;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/arena", metas::arena::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
