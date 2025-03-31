#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub mod errors;
pub mod formats;

use dirs::home_dir;
use errors::{ConfigError, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{canonicalize, create_dir_all, rename, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

pub trait Config: Serialize + DeserializeOwned + PartialEq + Default {
    /// The format to use for the config file.
    type FormatType: Format<Self::FormatContext>;

    /// The context for the format.
    type FormatContext: Default;

    /// The context for the format.
    fn format_context(&self) -> Self::FormatContext {
        Self::FormatContext::default()
    }

    /// The path and filename of the config file.
    ///
    /// ## Arguments
    ///
    /// * `home_dir` - The home directory of the user if needed.
    ///
    /// ## Returns
    ///
    /// * `Option<PathBuf>` - The path to the config file (parent directory), home directory will be used if `None` is returned.
    /// * `String` - The filename of the config file without the extension.
    fn config_path_and_filename(home_dir: &Path) -> (Option<PathBuf>, &str);

    /// Load the config from file.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use configura::{Config, load_config, formats::JsonFormat};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    /// struct ConfigData {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Config for ConfigData {
    ///     type FormatType = JsonFormat;
    ///     type FormatContext = ();
    ///
    ///     fn config_path_and_filename(_: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
    ///         (None, "config")
    ///     }
    /// }
    ///
    /// let mut data = ConfigData::default();
    /// data.load().unwrap();
    /// assert_eq!(data, ConfigData::default());
    ///
    /// data.name = "John".into();
    /// data.age = 30;
    ///
    /// data.save().unwrap();
    /// ```
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::Deserialization`]: Deserialization error
    /// - [`ConfigError::Io`]: IO error
    /// - [`ConfigError::NoHomeDir`]: No home directory found
    fn load(&mut self) -> Result<()> {
        let data: Self = load_config()?;
        *self = data;
        Ok(())
    }

    /// Save the config to file.
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::Deserialization`]: Deserialization error
    /// - [`ConfigError::Io`]: IO error
    /// - [`ConfigError::NoHomeDir`]: No home directory found
    fn save(&self) -> Result<()> {
        let path = self.path()?;
        let temp_path = path.with_file_name(format!(
            "{}.tmp",
            path.file_name()
                .map_or(String::new(), |f| f.to_string_lossy().to_string())
        ));

        if let Some(parent) = temp_path.parent() {
            create_dir_all(parent)?;
        }

        if temp_path.is_file() {
            return Err(ConfigError::FailedWrite(
                canonicalize(&temp_path)
                    .unwrap_or(temp_path.clone())
                    .display()
                    .to_string(),
            ));
        }

        let context = self.format_context();
        let data_str = Self::FormatType::to_string(self, false, Some(&context))?;

        match read_from_file(&path) {
            Ok(data) if data == data_str => return Ok(()),
            Ok(_) | Err(ConfigError::Io(_)) => (),
            Err(e) => return Err(e),
        }

        let temp_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)?;
        let mut writer = BufWriter::new(temp_file);

        writer.write_all(data_str.as_bytes())?;

        drop(writer);
        rename(temp_path, path)?;
        Ok(())
    }

    /// Convert the config data to a String based on the format.
    ///
    /// ## Arguments
    ///
    /// * `pretty` - Whether to format the output string (if supported by the format).
    ///
    /// ## Returns
    ///
    /// * `String` - The formatted string.
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::Serialization`]: Serialization error
    fn to_string(&self, pretty: bool) -> Result<String> {
        let context = self.format_context();
        Self::FormatType::to_string(self, pretty, Some(&context))
    }

    /// Get the path to the config file.
    ///
    /// ## Returns
    ///
    /// * `PathBuf` - The full path to the config file.
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::NoHomeDir`]: No home directory found
    fn path(&self) -> Result<PathBuf> {
        final_path::<Self>()
    }
}

pub trait Format<C> {
    /// The file extension for the config file (without the dot).
    const EXTENSION: &'static str;

    type FormatContext: Default;

    /// Serialize the config data to a string.
    ///
    /// ## Arguments
    ///
    /// * `data` - The data to serialize.
    /// * `pretty` - Whether to format the output string (if supported by the format).
    ///
    /// ## Returns
    ///
    /// * `String` - The serialized data.
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::Serialization`]: Serialization error
    fn to_string<T>(data: &T, pretty: bool, context: Option<&C>) -> Result<String>
    where
        T: Serialize;

    /// Deserialize the config data from a reader.
    ///
    /// ## Arguments
    ///
    /// * `reader` - The reader to deserialize from.
    ///
    /// ## Returns
    ///
    /// * `T` - The deserialized data.
    ///
    /// ## Errors
    ///
    /// - [`ConfigError::Deserialization`]: Deserialization error
    fn from_reader<R, T>(reader: R, context: Option<&C>) -> Result<T>
    where
        R: Read,
        T: DeserializeOwned;
}

/// Load the config data from file.
///
/// ## Example
///
/// ```rust,no_run
/// use configura::{Config, load_config, formats::JsonFormat};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
/// struct ConfigData {
///     name: String,
///     age: u8,
/// }
///
/// impl Config for ConfigData {
///     type FormatType = JsonFormat;
///     type FormatContext = ();
///
///     fn config_path_and_filename(_: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
///         (None, "config")
///     }
/// }
///
/// let mut data: ConfigData = load_config().unwrap();
/// assert_eq!(data, ConfigData::default());
///
/// data.name = "John".into();
/// data.age = 30;
///
/// data.save().unwrap();
/// ```
///
/// ## Errors
///
/// - [`ConfigError::Deserialization`]: Deserialization error
/// - [`ConfigError::Io`]: IO error
/// - [`ConfigError::NoHomeDir`]: No home directory found
pub fn load_config<T>() -> Result<T>
where
    T: Config,
{
    let path = final_path::<T>()?;
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(T::default());
        }
        Err(e) => return Err(e.into()),
    };

    let context = T::default().format_context();
    let data: T = T::FormatType::from_reader(BufReader::new(file), Some(&context))?;
    Ok(data)
}

/// Read the contents of a file into a String.
///
/// ## Arguments
///
/// * `path` - The path to the file.
///
/// ## Returns
///
/// * `String` - The contents of the file.
///
/// ## Errors
///
/// - [`ConfigError::Io`]: IO error
fn read_from_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    drop(reader);
    Ok(buffer)
}

/// Get the path to the config file.
///
/// ## Returns
///
/// * `PathBuf` - The full path to the config file.
///
/// ## Errors
///
/// - [`ConfigError::NoHomeDir`]: No home directory found
fn final_path<T>() -> Result<PathBuf>
where
    T: Config,
{
    let home = home_dir().ok_or(ConfigError::NoHomeDir)?;
    let (path, filename) = T::config_path_and_filename(&home);
    Ok(path
        .unwrap_or(home.clone())
        .join(format!("{filename}.{}", T::FormatType::EXTENSION)))
}

#[cfg(test)]
mod tests {
    use super::{load_config, Config, Result};
    use serde::{Deserialize, Serialize};
    use std::{fmt::Debug, fs::remove_file, path::PathBuf};
    use tempfile::tempdir;

    const TEST_NAME: &str = "Alice";
    const TEST_AGE: u8 = 30;
    const TEST_FILENAME: &str = "test_config";

    macro_rules! generate_format_test {
        ($format_name:ident, $format_type:path, $feature:literal) => {
            #[test]
            #[cfg(feature = $feature)]
            fn $format_name() -> Result<()> {
                #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
                struct TestConfig {
                    name: String,
                    age: u8,
                }

                impl Config for TestConfig {
                    type FormatType = $format_type;
                    type FormatContext = ();

                    fn config_path_and_filename(_: &std::path::Path) -> (Option<PathBuf>, &str) {
                        (None, TEST_FILENAME)
                    }
                }

                let temp_dir = tempdir()?;
                let temp_path = temp_dir.path().display().to_string();
                temp_env::with_vars(
                    vec![
                        ("HOME", Some(temp_path.clone())),
                        #[cfg(windows)]
                        ("USERPROFILE", Some(temp_path)),
                    ],
                    || {
                        run_test(&TestConfig {
                            name: TEST_NAME.to_string(),
                            age: TEST_AGE,
                        })
                    },
                )
            }
        };
    }

    generate_format_test!(test_config_json, super::formats::JsonFormat, "json");
    generate_format_test!(test_config_toml, super::formats::TomlFormat, "toml");
    generate_format_test!(test_config_yaml, super::formats::YamlFormat, "yaml");

    fn run_test<T>(original: &T) -> Result<()>
    where
        T: Config + Debug,
    {
        let loaded1: T = load_config()?;
        assert_eq!(loaded1, T::default());

        original.save()?;

        let loaded2: T = load_config()?;
        assert_eq!(&loaded2, original);

        let str = loaded2.to_string(true)?;
        assert!(str.contains(TEST_NAME));
        assert!(str.contains(&TEST_AGE.to_string()));

        remove_file(original.path()?)?;
        Ok(())
    }
}
