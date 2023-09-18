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
//!
use crate::config;
use base64ct::{Base64Url, Encoding};
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha512;

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    HmacKeyFailure,

    PasswordMismatch,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// -----------------------------------------------------------------------------
// Core
// -----------------------------------------------------------------------------

pub struct EncryptedContent {
    content: String, // Clear content.
    salt: String,    // Clear salt.
}

type HmacSha512 = Hmac<Sha512>;

fn encrypt_into_b64u(key: &[u8], encrypted_content: &EncryptedContent) -> Result<String> {
    let EncryptedContent { content, salt } = encrypted_content;

    // Create an HMAC-SHA-512 instance which uses `key` as the secret key.
    let mut mac = HmacSha512::new_from_slice(key).map_err(|_| Error::HmacKeyFailure)?;
    // Add the content to be encrypted.
    mac.update(content.as_bytes());
    mac.update(salt.as_bytes());
    // Finalise the HMAC-SHA-512 instance
    let mac_result = mac.finalize();

    Ok(Base64Url::encode_string(&mac_result.into_bytes()))
}

pub fn encrypt_password(encrypted_content: &EncryptedContent) -> Result<String> {
    let key = &config::get_config().PASSWORD_KEY;
    let enc = encrypt_into_b64u(key.as_bytes(), encrypted_content)?;

    Ok(format!("#01#{enc}"))
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
