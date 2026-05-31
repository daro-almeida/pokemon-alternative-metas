use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    adapters::repositories::postgres::PostgresMatchmakingRepository,
    application::{AppResult, repositories::matchmaking::MatchmakingRepository},
    domain::arena::{arena_match::ArenaMatch, run::ArenaRunInfo},
};

#[async_trait]
impl MatchmakingRepository for PostgresMatchmakingRepository {
    async fn get_matches(&self, username: &str) -> AppResult<Vec<ArenaMatch>> {
        todo!()
    }

    async fn create_match(
        &self,
        run_info_1: &ArenaRunInfo,
        run_info_2: &ArenaRunInfo,
    ) -> AppResult<ArenaMatch> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO arena_matches (run_id_1, run_id_2)
            VALUES ($1, $2)
            "#,
            run_info_1.run_id,
            run_info_2.run_id
        )
        .execute(&mut *tx)
        .await?;

        let opponent = sqlx::query_scalar!(
            r#"
            SELECT username FROM runs WHERE run_id = $1
            "#,
            run_info_2.run_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ArenaMatch {
            opponent,
            wins: 0,
            losses: 0,
            won: None,
        })
    }

    async fn have_runs_played(&self, run_id_1: &Uuid, run_id_2: &Uuid) -> AppResult<bool> {
        let result = sqlx::query_scalar!(
            r#"
        SELECT EXISTS(
            SELECT 1 FROM arena_matches
            WHERE (run_id_1 = $1 AND run_id_2 = $2)
            OR (run_id_1 = $2 AND run_id_2 = $1)
        )
        "#,
            run_id_1,
            run_id_2
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }
    async fn abandon_match(
        &self,
        match_id: &Uuid,
        username: &str,
        elo_change: i32,
    ) -> AppResult<()> {
        todo!()
    }
}
