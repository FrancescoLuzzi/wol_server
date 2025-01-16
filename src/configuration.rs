use config::{Config, ConfigError, File};
use secrecy::SecretString;
use serde::Deserialize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::convert::TryFrom;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use tracing_log::log::Level;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub base_url: String,
    pub host: IpAddr,
    pub port: u16,
    pub hmac_secret: SecretString,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    location: PathBuf,
    #[serde(deserialize_with = "journal_from_string")]
    journal_mode: SqliteJournalMode,
}

fn journal_from_string<'de, D>(deserializer: D) -> Result<SqliteJournalMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: std::borrow::Cow<String> = Deserialize::deserialize(deserializer)?;
    SqliteJournalMode::from_str(s.as_str()).map_err(|e| {
        serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&e.to_string()),
            &r#""delete", "truncate", "persist", "memory", "wal" and "off" values supported"#,
        )
    })
}

impl DatabaseSettings {
    pub fn in_memory(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new().in_memory(true)
    }

    pub fn on_file(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
            .filename(&self.location)
            .journal_mode(SqliteJournalMode::Wal)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct LoggingSettings {
    pub enabled: bool,
    #[serde(deserialize_with = "level_from_string")]
    pub level: Level,
}

fn level_from_string<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: std::borrow::Cow<String> = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "debug" => Ok(Level::Debug),
        "info" => Ok(Level::Info),
        "warn" => Ok(Level::Warn),
        "error" => Ok(Level::Error),
        default => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(default),
            &r#""debug", "info", "warn" or "error" values supported"#,
        )),
    }
}
enum Environment {
    Local,
    Dev,
    Prod,
}

impl Environment {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Dev => "dev",
            Self::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            err_env => Err(format!("no such Environment supported: {err_env}")),
        }
    }
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP__ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP__ENVIRONMENT.");
    let environment_filename = format!("{}.toml", environment.as_str());
    let settings = Config::builder()
        .add_source(File::from(configuration_directory.join("base.toml")))
        .add_source(File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP__APPLICATION_PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("__")
                .separator("_"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
