use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArenaConfig {
    pub num_picks: usize,
    pub num_buckets: usize,
    pub points_to_bucket: HashMap<usize, usize>,
    pub options_per_bucket: Vec<usize>,
    pub quotas: Vec<usize>,
}
