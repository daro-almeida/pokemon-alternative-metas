use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub mod pokemon;
pub mod arena;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct TimeFormat(String);

impl TimeFormat {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<OffsetDateTime> for TimeFormat {
    type Error = anyhow::Error;

    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        Ok(Self(
            value.format(&Rfc3339).map_err(|err| anyhow::anyhow!(err))?,
        ))
    }
}