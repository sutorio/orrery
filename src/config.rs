//! Environment-variable based configuration
//!
//! Cargo, via the `[env]` section of a `config.toml` file in the `.cargo` directory,
//! allows you to set environment variables that will be used when running cargo commands.
//!
//! NOTE: these are *only* used when running cargo commands. They are not used when running
//! the application directly, nor are they available to other runtimes during development.
//!
use envconfig::Envconfig;
use std::sync::OnceLock;

/// The config need only be loaded once, hence definition as a `static`.
/// Wrapped in a function to scope the visibility; technically makes little
/// difference, as `static`s will always be global.
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        // Note that this explicity panics if the config cannot be loaded: this
        // is the desired behaviour in this instance - the application cannot
        // run without a config.
        Config::init_from_env().unwrap_or_else(|exception| {
            panic!("Failed to load config from environment: {exception:?}");
        })
    })
}

// NOTE: the config keys use SCREAMING_SNAKE_CASE to match the environment variables.
#[allow(non_snake_case)]
#[derive(Debug, Envconfig)]
pub struct Config {
    /// The URL of the database to connect to. Sqlx requires this, both for connection & for the CLI to work.
    pub DATABASE_URL: String,
    /// The log level to use for the application.
    pub RUST_LOG: String,
    /// The port to listen on for HTTP requests.
    pub SERVER_PORT: u16,
    /// The path to the folder containing the static files to serve.
    pub ASSETS_FOLDER: String,
    /// TODO: document
    pub PASSWORD_KEY: String,
    /// TODO: document
    pub TOKEN_KEY: String,
    /// TODO: document
    pub TOKEN_DURATION_IN_SECONDS: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // create a hash map using iterators, passing in the pairs from a vec
    fn create_config_map(vec: Vec<(&str, &str)>) -> HashMap<String, String> {
        vec.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn using_config_map_in_tests() {
        let required_configs = vec![
            ("DATABASE_URL", ":memory:"),
            ("RUST_LOG", "debug"),
            ("SERVER_PORT", "12345"),
            ("ASSETS_FOLDER", "assets"),
            ("PASSWORD_KEY", "password"),
            ("TOKEN_KEY", "token"),
            ("TOKEN_DURATION_IN_SECONDS", "3600"),
        ];
        // Create a HashMap that looks like the required environment.
        let mock_env = create_config_map(required_configs);
        // Initialise the config from HashMap, avoiding race conditions.
        let config = Config::init_from_hashmap(&mock_env).unwrap();

        assert_eq!(config.DATABASE_URL, ":memory:");
        assert_eq!(config.RUST_LOG, "debug");
        assert_eq!(config.SERVER_PORT, 12345u16);
        assert_eq!(config.ASSETS_FOLDER, "assets");
        assert_eq!(config.PASSWORD_KEY, "password");
        assert_eq!(config.TOKEN_KEY, "token");
        assert_eq!(config.TOKEN_DURATION_IN_SECONDS, 3600u32);
    }
}
