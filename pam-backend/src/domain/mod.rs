use anyhow::Context;
use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub mod pokemon;
pub mod arena;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct TimeFormat(String);

impl TimeFormat {
    pub fn new(timestamp: OffsetDateTime) -> anyhow::Result<Self> {
        Ok(Self(
            timestamp.format(&Rfc3339).context("Invalid timestamp")?,
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}