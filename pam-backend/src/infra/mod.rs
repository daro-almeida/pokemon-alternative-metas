use crate::{adapters::persistence::PostgresPersistence, infra::db::init_pg_db};

pub mod app;
pub mod config;
pub mod db;
pub mod setup;

pub async fn postgres_persistence() -> anyhow::Result<PostgresPersistence> {
    let pool = init_pg_db().await?;
    let persistence = PostgresPersistence::new(pool);
    Ok(persistence)
}