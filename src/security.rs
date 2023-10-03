use crate::config;
use serde::Serialize;
use std::{fmt::Display, str::FromStr};
use sutorio_axum_utils_crypto as crypto;

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    // Format errors
    Crypto(crypto::Error),
}

impl From<crypto::Error> for Error {
	fn from(val: crypto::Error) -> Self {
		Self::Crypto(val)
	}
} 

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}


// -----------------------------------------------------------------------------
// Tokens
// -----------------------------------------------------------------------------

pub fn generate_web_token(username: &str, salt: &str) -> Result<Token> {
    let config = config::get_config();
    crypto::generate_token(
        username,
        config.TOKEN_DURATION_IN_SECONDS,
        salt,
        config.TOKEN_KEY.as_bytes(),
    )
}

pub fn validate_web_token(original_token: &Token, salt: &str) -> Result<()> {
    let config = config::get_config();
    crypto::validate_token_signature_and_expiration(original_token, salt, config.TOKEN_KEY.as_bytes())
}


