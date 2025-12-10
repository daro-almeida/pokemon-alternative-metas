use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ArenaRun {
    run_id: Uuid,
    wins: i32,
    losses: i32,
}

#[derive(Debug, FromRow)]
pub struct ArenaTeam {
    run_id: Uuid,
    username: String,
    pick_no: i32,
    pokemon: String,
}

#[derive(Debug, FromRow)]
pub struct ArenaPick {
    username: String,
    option_no: i32,
    pokemon: String,
}
