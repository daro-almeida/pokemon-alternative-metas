use serde::Serialize;
use sqlx::types::time::PrimitiveDateTime;
use uuid::Uuid;

use crate::domain::pokemon::Pokemon;

pub type Bucket = usize;

#[derive(Debug, Serialize)]
pub struct  Pick {
    pub pick_num: usize,
    pub options: Vec<&'static Pokemon>,
}

#[derive(Debug, Serialize)]
pub struct ArenaRunInfo {
    #[serde(skip_serializing)]
    pub run_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub wins: u32,
    pub losses: u32,
    pub finished_draft: bool,
    pub team: Vec<&'static Pokemon>,
    #[serde(skip_serializing)]
    pub team_buckets: Vec<Bucket>,
}