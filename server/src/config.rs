use std::path::{Path, PathBuf};

pub use config::ConfigError;
use config::{Config as RawConfig, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct HttpConfig {
    #[serde(default = "HttpConfig::default_host")]
    pub host: String,

    #[serde(default = "HttpConfig::default_port")]
    pub port: u16,

    #[serde(default = "HttpConfig::default_upload_file_limit")]
    pub upload_file_limit: usize,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            host: Self::default_host(),
            port: Self::default_port(),
            upload_file_limit: Self::default_upload_file_limit(),
        }
    }
}

impl HttpConfig {
    pub const fn default_upload_file_limit() -> usize {
        2 * 1024 * 1024 * 1024
    }

    pub fn default_host() -> String {
        "localhost".into()
    }

    pub const fn default_port() -> u16 {
        3080
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LoggerConfig {
    #[serde(default = "LoggerConfig::default_spec")]
    pub spec: String,

    pub path: Option<PathBuf>,

    pub duplicate_to_stdout: bool,

    #[serde(default = "LoggerConfig::default_keep_log_for_days")]
    pub keep_log_for_days: usize,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            spec: Self::default_spec(),
            path: None,
            duplicate_to_stdout: false,
            keep_log_for_days: Self::default_keep_log_for_days(),
        }
    }
}

impl LoggerConfig {
    fn default_spec() -> String {
        "info".into()
    }

    const fn default_keep_log_for_days() -> usize {
        7
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpConfig,
    pub log: LoggerConfig,
}

impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config = RawConfig::builder()
            .add_source(File::from(path.as_ref()))
            // Add in settings from the environment (with a prefix of BLOGEAR)
            // Eg.. `BLOGEAR__HTTP__PORT=8090 blogear_server` would set the `http.port` key
            .add_source(
                Environment::with_prefix("BLOGEAR")
                    .separator("__")
                    .list_separator(",")
                    .try_parsing(true),
            )
            .build()?;
        config.try_deserialize()
    }
}
