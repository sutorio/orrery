//! Security related functionality.
//!
//! Password hashing, validation etc. This tries to rely on the [RustCrypto](https://github.com/RustCrypto)
//! organisation's crates as much as possible. I have absolutely no wish to
//! roll my own crypto/auth, so I am following the instructions laid out in the
//! respective RustCrypto repositories as closely as possible.
//!
//! It is extremely important that the documentation is read and understood
//! before any changes are made to this code: for quick reference, see:
//!
//! - [hmac](https://docs.rs/hmac/latest/hmac/)
//! - [sha2](https://docs.rs/sha2/latest/sha2/)
//! - [base64ct](https://docs.rs/base64ct/latest/base64ct/)
//!
//! TODO: move to seperate crate in a workspace. This is entirely reusable, and has
//!       no ties to any specific application.
use crate::config;
use base64ct::Encoding;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha512;
use std::{fmt::Display, str::FromStr};

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    // Format errors
    B64DecodingFailure,
    UTCParsingFailure(String),
    // Key-related errors
    HmacKeyFailure,
    // Password-related errors
    PasswordMismatch,
    // Token-related errors
    TokenFormatInvalid,
    TokenIdentifierDecodeFailure,
    TokenExpirationDecodeFailure,
    TokenSignatureMismatch,
    TokenExpirationNotIso,
    TokenExpired,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// -----------------------------------------------------------------------------
// Base64 encoding/decoding
//
// NOTE: url-safe base64 encoding is used.
// REVIEW: should the *un*padded b64u encoding be used? Currently the *padded* version is used.
// -----------------------------------------------------------------------------

pub struct UrlSafeBase64;

impl UrlSafeBase64 {
    pub fn encode(content: &[u8]) -> String {
        base64ct::Base64Url::encode_string(content)
    }

    pub fn decode(content: &str) -> Result<String> {
        match base64ct::Base64Url::decode_vec(content) {
            Ok(byte_vec) => String::from_utf8(byte_vec)
                .map(|s| s.to_string())
                .map_err(|_| Error::B64DecodingFailure),
            Err(_) => Err(Error::B64DecodingFailure),
        }
    }
}

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
    chrono::DateTime::parse_from_rfc3339(moment)
        .map_err(|_| Error::UTCParsingFailure(moment.to_string()))
}

// -----------------------------------------------------------------------------
// Core
// -----------------------------------------------------------------------------

pub struct EncryptedContent {
    pub content: String, // Clear content.
    pub salt: String,    // Clear salt.
}

pub type HmacSha512 = Hmac<Sha512>;

impl EncryptedContent {
    fn into_b64u(&self, key: &[u8]) -> Result<String> {
        let mut mac = HmacSha512::new_from_slice(key).map_err(|_| Error::HmacKeyFailure)?;
        // Add the content to be encrypted.
        mac.update(self.content.as_bytes());
        mac.update(self.salt.as_bytes());
        // Finalise the HMAC-SHA-512 instance
        let mac_result = mac.finalize();

        Ok(UrlSafeBase64::encode(&mac_result.into_bytes()))
    }
}

// -----------------------------------------------------------------------------
// Passwords
// -----------------------------------------------------------------------------

pub fn encrypt_password(encrypted_content: &EncryptedContent) -> Result<String> {
    let result = encrypted_content.into_b64u(config::get_config().PASSWORD_KEY.as_bytes())?;

    Ok(format!("#01#{result}"))
}

pub fn validate_password(
    encrypted_content: &EncryptedContent,
    password_reference: &str,
) -> Result<()> {
    let password = encrypt_password(encrypted_content)?;

    if password == password_reference {
        Ok(())
    } else {
        Err(Error::PasswordMismatch)
    }
}

// -----------------------------------------------------------------------------
// Tokens
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Token {
    pub identifier: String,
    pub expiration: String,
    pub signature: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            UrlSafeBase64::encode(&self.identifier.as_bytes()),
            UrlSafeBase64::encode(&self.expiration.as_bytes()),
            self.signature,
        )
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_string: &str) -> std::result::Result<Self, Self::Err> {
        let chunks: Vec<&str> = token_string.split(".").collect();

        if chunks.len() != 3 {
            return Err(Error::TokenFormatInvalid);
        }

        let identifier =
            UrlSafeBase64::decode(chunks[0]).map_err(|_| Error::TokenIdentifierDecodeFailure)?;
        let expiration =
            UrlSafeBase64::decode(chunks[1]).map_err(|_| Error::TokenExpirationDecodeFailure)?;
        let signature = chunks[2].to_string();

        Ok(Token {
            identifier,
            expiration,
            signature,
        })
    }
}

pub fn generate_web_token(username: &str, salt: &str) -> Result<Token> {
    let config = config::get_config();
    _generate_token(
        username,
        config.TOKEN_DURATION_IN_SECONDS,
        salt,
        config.TOKEN_KEY.as_bytes(),
    )
}

pub fn validate_web_token(original_token: &Token, salt: &str) -> Result<()> {
    let config = config::get_config();
    _validate_token_signature_and_expiration(original_token, salt, config.TOKEN_KEY.as_bytes())
}

fn _generate_token(
    identifier: &str,
    duration_in_seconds: f64,
    salt: &str,
    key: &[u8],
) -> Result<Token> {
    let identifier = identifier.to_string();
    let expiration = now_utc_plus_seconds_string(duration_in_seconds);
    let signature = _token_sign_into_b64u(&identifier, &expiration, salt, key)?;

    Ok(Token {
        identifier,
        expiration,
        signature,
    })
}

fn _validate_token_signature_and_expiration(
    original_token: &Token,
    salt: &str,
    key: &[u8],
) -> Result<()> {
    let new_signature = _token_sign_into_b64u(
        &original_token.identifier,
        &original_token.expiration,
        salt,
        key,
    )?;

    // Validate the signature.
    if new_signature != original_token.signature {
        return Err(Error::TokenSignatureMismatch);
    }

    // Validate the expiration.
    let original_expiration =
        parse_utc(&original_token.expiration).map_err(|_| Error::TokenExpirationNotIso)?;

    if original_expiration < now_utc() {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

/// Create token signature from token parts + salt
fn _token_sign_into_b64u(
    identifier: &str,
    expiration: &str,
    salt: &str,
    key: &[u8],
) -> Result<String> {
    let content = format!(
        "{}.{}",
        UrlSafeBase64::encode(identifier.as_bytes()),
        UrlSafeBase64::encode(expiration.as_bytes())
    );
    let signature = EncryptedContent {
        content,
        salt: salt.to_string(),
    }
    .into_b64u(key)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_token_display_impl() -> Result<()> {
        let stringified_input_token = Token {
            identifier: "identifier".to_string(),
            expiration: "expiration".to_string(),
            signature: "signature".to_string(),
        }
        .to_string();

        let expected_output_string = "aWRlbnRpZmllcg==.ZXhwaXJhdGlvbg==.signature";

        assert_eq!(stringified_input_token, expected_output_string);

        Ok(())
    }

    #[test]
    fn test_token_from_string_impl() -> Result<()> {
        let parsed_input_string: Token = "aWRlbnRpZmllcg==.ZXhwaXJhdGlvbg==.signature".parse()?;

        let expected_output_token = Token {
            identifier: "identifier".to_string(),
            expiration: "expiration".to_string(),
            signature: "signature".to_string(),
        };

        // NOTE: `Token` does not implement `PartialEq` by design -- it would be implemented for testing
        //       purposes only, so deriving that would be confusing. It _does_ implement `Debug`, so
        //       we can use the `:?` format specifier to compare the two instances.
        assert_eq!(
            format!("{parsed_input_string:?}"),
            format!("{expected_output_token:?}")
        );

        Ok(())
    }

    #[test]
    fn test_validate_web_token() -> Result<()> {
        let input_user = "user";
        let input_salt = "salt";
        let input_duration_in_seconds = 0.02; // 20ms
        let input_key = &config::get_config().TOKEN_KEY.as_bytes();

        let input_token =
            _generate_token(input_user, input_duration_in_seconds, input_salt, input_key)?;

        thread::sleep(Duration::from_millis(10));

        let expected_result = validate_web_token(&input_token, &input_salt);

        expected_result?;

        Ok(())
    }

    #[test]
    fn test_validate_web_token_expired() -> Result<()> {
        let input_user = "user";
        let input_salt = "salt";
        let input_duration_in_seconds = 0.01; // 10ms
        let input_key = &config::get_config().TOKEN_KEY.as_bytes();

        let input_token =
            _generate_token(input_user, input_duration_in_seconds, input_salt, input_key)?;

        thread::sleep(Duration::from_millis(20));

        let expected_result = validate_web_token(&input_token, &input_salt);

        assert!(matches!(expected_result, Err(Error::TokenExpired)));

        Ok(())
    }
}
