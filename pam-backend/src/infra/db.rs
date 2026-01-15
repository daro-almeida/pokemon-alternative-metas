
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_pg_db() -> anyhow::Result<Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
