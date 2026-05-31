use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct ArenaRunInfoDao {
    pub run_id: Uuid,
    pub username: String,
    pub created_at: OffsetDateTime,
    pub wins: i32,
    pub losses: i32,
    pub finished_draft: bool,
    pub team: Vec<String>,
    pub team_buckets: Vec<i32>,
}

#[derive(FromRow, Debug)]
pub struct ArenaMatchDao {
    pub match_id: Uuid,
    pub run_id_1: Uuid,
    pub run_id_2: Uuid,
    pub finished: bool,
    pub created_at: OffsetDateTime,
    pub run_1_wins: i32,
    pub run_2_wins: i32,
    pub winner_run_id: Option<Uuid>,
}

