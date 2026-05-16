use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChoosePickRequest {
    pub option_no: usize,
}
