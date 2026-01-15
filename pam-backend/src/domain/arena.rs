use sqlx::FromRow;



#[derive(FromRow, Debug, Serialize)]
pub struct ArenaRunInfo {
    pub run_id: Uuid,
    pub username: String,
    pub wins: i32,
    pub losses: i32,
    pub pool: Vec<String>,
    pub pool_buckets: Vec<i32>,
}
