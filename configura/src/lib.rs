//! # configura

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations)]

pub mod format;

use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

/// Represents the config file structure
pub trait ConfigData: Serialize + for<'de> Deserialize<'de> + Sized {}

impl<T> ConfigData for T where T: Serialize + for<'de> Deserialize<'de> + Sized {}

/// Represents a config file format
pub trait Format: Sized {
    /// The error type returned by the format
    type Error;

    /// The format configuration/auxiliary data
    type Config;

    fn default_wrapper<T>() -> ConfigWrapper<T, Self::Config>
    where
        T: ConfigData + Default,
        Self::Config: Default,
    {
        ConfigWrapper {
            path: None,
            data: T::default(),
            format_config: Self::Config::default(),
        }
    }

    /// Serializes the config data into a byte array
    ///
    /// ## Errors
    ///
    /// Returns an error if serialization fails
    fn serialize<T>(&self, data: &T, pretty: bool) -> Result<Vec<u8>, Self::Error>
    where
        T: ConfigData;

    /// Deserializes the config data from a byte array
    ///
    /// ## Errors
    ///
    /// Returns an error if deserialization fails
    fn deserialize<T>(&self, data: &[u8]) -> Result<T, Self::Error>
    where
        T: ConfigData;
}

/// A wrapper for the config data and the format.
///
/// This is the entry point for the library
#[derive(Debug)]
pub struct ConfigWrapper<T, F>
where
    F: Format,
{
    path: Option<PathBuf>,
    data: T,
    format_config: F::Config,
}

impl<T, F> Deref for ConfigWrapper<T, F>
where
    T: ConfigData,
    F: Format,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, F> DerefMut for ConfigWrapper<T, F>
where
    T: ConfigData,
    F: Format,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, F> ConfigWrapper<T, F>
where
    T: ConfigData,
    F: Format,
    F::Config: Format + Default,
{
    pub fn new<P>(path: P) -> Self
    where
        T: Default,
        P: AsRef<Path>,
    {
        Self {
            path: Some(path.as_ref().to_path_buf()),
            data: T::default(),
            format_config: F::Config::default(),
        }
    }

    #[must_use]
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub fn with_data(mut self, data: T) -> Self {
        self.data = data;
        self
    }

    #[must_use]
    pub fn with_format_config(mut self, format_config: F::Config) -> Self {
        self.format_config = format_config;
        self
    }
}
