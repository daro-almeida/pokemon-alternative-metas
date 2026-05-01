use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ArenaMatch {
    pub opponent: String,
    pub wins: u32,
    pub losses: u32,
    pub won: Option<bool>,
}