use super::super::{ConfigData, Format};

#[derive(Debug, Default)]
pub struct JsonFormat;

impl Format for JsonFormat {
    type Error = serde_json::Error;
    type Config = Self;

    fn serialize<T>(&self, data: &T, pretty: bool) -> Result<Vec<u8>, Self::Error>
    where
        T: ConfigData,
    {
        if pretty {
            serde_json::to_string_pretty(data).map(std::string::String::into_bytes)
        } else {
            serde_json::to_string(data).map(std::string::String::into_bytes)
        }
    }

    fn deserialize<T>(&self, data: &[u8]) -> Result<T, Self::Error>
    where
        T: ConfigData,
    {
        serde_json::from_slice(data)
    }
}
