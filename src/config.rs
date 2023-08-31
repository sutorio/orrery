///! # Config
///!
///! 1. CLI/Env based pre-startup config
///! 2. Logging/tracing configuration
///!
///! It seems to make sense to keep these two concerns colocated, as they're both
///! pre-startup configuration concerns.
use clap::Parser;

// ---------------------------------------------------------------------------------
// 1. CLI/Env based pre-startup config
// ---------------------------------------------------------------------------------

/// Arguments can be passed to the binary on startup to override the defaults.
///
/// Clap is used as an argparser, and it also supports pulling from env vars, which
/// is useful.
#[derive(Parser, Clone, Debug)]
pub struct AppConfig {
    /// The connection URL for the SQLite database
    #[clap(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Logging/tracing level. NOTE: this can be granular, specifying levels for specific deps
    #[clap(long, env = "RUST_LOG")]
    pub rust_log: String,

    /// The port to serve the application on
    #[clap(long, env = "SERVER_PORT")]
    pub server_port: u16,
}

/// Parse the CLI args and env vars into a config struct.
/// This occurs in two steps, and requires the `env` feature to be turned on for Clap:
///
/// 1. Load in env vars from a .env file. When the app is deployed there will be no .env file present:
///    any required env vars will be set in the environment.
/// 2. Parse the CLI args and env vars into a config struct. Clap's `env` feature will pull in the
///    env vars: if they aren't found, the app will exit with a helpful message.
pub fn parse_app_config() -> AppConfig {
    dotenvy::dotenv().ok(); // [1]
    AppConfig::parse() // [2]
}

// ---------------------------------------------------------------------------------
// 2. Logging/tracing configuration
// ---------------------------------------------------------------------------------

/// Tracing/structured logging is fuck on. The docs IMO are never very good, and it takes
/// some fiddling and several different libraries.
///
/// Anyway, `tracing` and `tracing_subscriber` are used here.
/// https://stackoverflow.com/questions/75009289/how-to-enable-logging-tracing-with-axum
/// https://stackoverflow.com/questions/74302133/how-to-log-and-filter-requests-with-axum-tokio
/// https://stackoverflow.com/questions/70013172/how-to-use-the-tracing-library
pub fn init_tracing() {
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "orrery=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
