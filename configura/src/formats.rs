#[cfg(feature = "json")]
mod json_impl {
    use crate::{Format, errors::ConfigError};
    use serde::{Serialize, de::DeserializeOwned};
    use serde_json::{from_reader, to_string, to_string_pretty};
    use std::io::Read;

    pub struct JsonFormat;

    impl Format<()> for JsonFormat {
        const EXTENSION: &'static str = "json";

        type FormatContext = ();

        fn to_string<T>(data: &T, pretty: bool, _context: Option<&()>) -> crate::Result<String>
        where
            T: Serialize,
        {
            if pretty {
                to_string_pretty(data)
            } else {
                to_string(data)
            }
            .map_err(|e| ConfigError::serialization(Self::EXTENSION, e))
        }

        fn from_reader<R, T>(reader: R, _context: Option<&()>) -> crate::Result<T>
        where
            R: Read,
            T: DeserializeOwned,
        {
            from_reader(reader).map_err(|e| ConfigError::deserialization(Self::EXTENSION, e))
        }
    }
}

#[cfg(feature = "json")]
pub use json_impl::JsonFormat;

#[cfg(feature = "toml")]
mod toml_impl {
    use crate::{Format, errors::ConfigError};
    use serde::{Serialize, de::DeserializeOwned};
    use std::io::{BufReader, Read};
    use toml::{from_str, to_string, to_string_pretty};

    pub struct TomlFormat;

    impl Format<()> for TomlFormat {
        const EXTENSION: &'static str = "toml";

        type FormatContext = ();

        fn to_string<T>(data: &T, pretty: bool, _context: Option<&()>) -> crate::Result<String>
        where
            T: Serialize,
        {
            if pretty {
                to_string_pretty(data)
            } else {
                to_string(data)
            }
            .map_err(|e| ConfigError::serialization(Self::EXTENSION, e))
        }

        fn from_reader<R, T>(reader: R, _context: Option<&()>) -> crate::Result<T>
        where
            R: Read,
            T: DeserializeOwned,
        {
            let mut buffer = String::new();
            let mut buf_reader = BufReader::new(reader);

            buf_reader.read_to_string(&mut buffer)?;
            from_str(&buffer).map_err(|e| ConfigError::deserialization(Self::EXTENSION, e))
        }
    }
}

#[cfg(feature = "toml")]
pub use toml_impl::TomlFormat;

#[cfg(feature = "yaml")]
mod yaml_impl {
    use crate::{Format, errors::ConfigError};
    use serde::{Serialize, de::DeserializeOwned};
    use serde_yml::{from_reader, to_string};
    use std::io::Read;

    pub struct YamlFormat;

    impl Format<()> for YamlFormat {
        const EXTENSION: &'static str = "yaml";

        type FormatContext = ();

        fn to_string<T>(data: &T, _pretty: bool, _context: Option<&()>) -> crate::Result<String>
        where
            T: Serialize,
        {
            to_string(data).map_err(|e| ConfigError::serialization(Self::EXTENSION, e))
        }

        fn from_reader<R, T>(reader: R, _context: Option<&()>) -> crate::Result<T>
        where
            R: Read,
            T: DeserializeOwned,
        {
            from_reader(reader).map_err(|e| ConfigError::deserialization(Self::EXTENSION, e))
        }
    }
}

#[cfg(feature = "yaml")]
pub use yaml_impl::YamlFormat;
