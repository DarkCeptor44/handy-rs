use std::fmt::Display;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    FailedWrite(String),
    Serialization(String, String),
    Deserialization(String, String),
    NoHomeDir,
}

impl std::error::Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "failed to read configuration file: {e}"),
            ConfigError::FailedWrite(e) => write!(f, "previous write failed: {e}"),
            ConfigError::Serialization(format, e) => {
                write!(f, "failed to serialize {} data: {e}", format.to_uppercase())
            }
            ConfigError::Deserialization(format, e) => write!(
                f,
                "failed to deserialize {} data: {e}",
                format.to_uppercase()
            ),
            ConfigError::NoHomeDir => write!(f, "home directory not found"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}

impl PartialEq for ConfigError {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), |(
            ConfigError::NoHomeDir,
            ConfigError::NoHomeDir,
        )| (
            ConfigError::FailedWrite(_),
            ConfigError::FailedWrite(_)
        ) | (
            ConfigError::Serialization(_, _),
            ConfigError::Serialization(_, _)
        ) | (
            ConfigError::Deserialization(_, _),
            ConfigError::Deserialization(_, _)
        ) | (
            ConfigError::Io(_),
            ConfigError::Io(_)
        ))
    }
}

impl Eq for ConfigError {}

impl ConfigError {
    pub fn serialization(format: &'static str, error: impl Display) -> Self {
        ConfigError::Serialization(format.into(), error.to_string())
    }

    pub fn deserialization(format: &'static str, error: impl Display) -> Self {
        ConfigError::Deserialization(format.into(), error.to_string())
    }
}
