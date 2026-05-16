use std::fmt;

use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct TimeFormat(String);

impl TimeFormat {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct TimeConversionError(pub String);

impl fmt::Display for TimeConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time conversion error: {}", self.0)
    }
}

impl std::error::Error for TimeConversionError {}

impl TryFrom<OffsetDateTime> for TimeFormat {
    type Error = TimeConversionError;
    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        value
            .format(&Rfc3339)
            .map(TimeFormat)
            .map_err(|e| TimeConversionError(e.to_string()))
    }
}
