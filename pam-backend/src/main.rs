use dotenvy::dotenv;

use crate::startup::{app::create_app, init_app_state};

pub mod adapters;
pub mod application;
pub mod domain;
pub mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app_state = init_app_state().await?;

    let app = create_app(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
