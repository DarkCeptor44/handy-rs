#[test]
#[cfg(feature = "json")]
fn test_config_json() {
    use configura::{format::JsonFormat, ConfigWrapper, Format};
    use serde::{Deserialize, Serialize};
    use tempfile::{tempdir, NamedTempFile};

    #[derive(Debug, Serialize, Deserialize)]
    struct MyConfig {
        value: i32,
        name: String,
    }

    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let config = JsonFormat::default_wrapper::<MyConfig>().with_path(temp_file.path());
}
