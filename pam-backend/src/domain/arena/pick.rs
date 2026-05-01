use serde::Serialize;

use crate::domain::pokemon::Pokemon;

#[derive(Debug, Serialize)]
pub struct Pick {
    pub pick_num: usize,
    pub options: Vec<&'static Pokemon>,
}