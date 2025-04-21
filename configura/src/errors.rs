use std::fmt::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("failed to read configuration file: {0}")]
    Io(String),

    #[error("previous write failed: {0}")]
    FailedWrite(String),

    #[error("failed to serialize {0} data: {1}")]
    Serialization(String, String),

    #[error("failed to deserialize {0} data: {1}")]
    Deserialization(String, String),

    #[error("home directory not found")]
    NoHomeDir,
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value.to_string())
    }
}

impl ConfigError {
    pub fn serialization(format: &'static str, error: impl Display) -> Self {
        ConfigError::Serialization(format.into(), error.to_string())
    }

    pub fn deserialization(format: &'static str, error: impl Display) -> Self {
        ConfigError::Deserialization(format.into(), error.to_string())
    }
}
