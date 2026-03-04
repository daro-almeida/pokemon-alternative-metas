use serde::Serialize;
use uuid::Uuid;

use crate::domain::{TimeFormat, pokemon::Pokemon};

pub type Bucket = usize;

#[derive(Debug, Serialize)]
pub struct Pick {
    pub pick_num: usize,
    pub options: Vec<&'static Pokemon>,
}


#[derive(Debug, Serialize)]
pub struct ArenaMatch {
    pub opponent: String,
    pub wins: u32,
    pub losses: u32,
    pub won: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ArenaRunInfo {
    #[serde(skip_serializing)]
    pub run_id: Uuid,
    pub created_at: TimeFormat,
    pub wins: u32,
    pub losses: u32,
    pub finished_draft: bool,
    pub matches: Vec<ArenaMatch>,
    pub team: Vec<&'static Pokemon>,
    #[serde(skip_serializing)]
    pub team_buckets: Vec<Bucket>,
}

impl ArenaRunInfo {
    pub fn new(run_id: Uuid, created_at: TimeFormat) -> Self {
        Self {
            run_id,
            created_at,
            wins: 0,
            losses: 0,
            finished_draft: false,
            matches: Vec::new(),
            team: Vec::new(),
            team_buckets: Vec::new(),
        }
    }
}
