use base64ct::Encoding;
use serde::Serialize;

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    ParseError(String),
    B64DecodingFailure,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// -----------------------------------------------------------------------------
// Timing
// -----------------------------------------------------------------------------

pub fn now_utc() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

pub fn format_utc_time(time: chrono::DateTime<chrono::Utc>) -> String {
    time.to_rfc3339()
}

pub fn now_utc_plus_seconds_string(seconds: f64) -> String {
    format_utc_time(now_utc() + chrono::Duration::seconds(seconds as i64))
}

pub fn parse_utc(moment: &str) -> Result<chrono::DateTime<chrono::FixedOffset>> {
    chrono::DateTime::parse_from_rfc3339(moment).map_err(|_| Error::ParseError(moment.to_string()))
}
