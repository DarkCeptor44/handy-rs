# configura

**configura** (Portuguese for "configure") is a Rust crate that provides convenient trait-based configuration loading and saving logic. This allows you to add configuration files to any Rust project, providing the file name, the format (which is JSON by default), and the path to the folder where the config file will be stored (which if not specified defaults to the home directory of the machine). A mirror can also be specified to store a copy of the config file in a different location with a different filename.

## Concepts

* **Config**: A trait that represents a config file, or rather the data that is stored in the config file.
* **Format**: A trait that represents the format of the config file, such as JSON, TOML, or YAML, and implements the necessary methods to serialize and deserialize the data.
* **FormatType**: A struct that implements the `Format` trait and it's methods (e.g. `JsonFormat`).
* **FormatContext**: A struct that holds the context for the format, such as encryption keys or other options the format implementation might need.

## Features

This crate provides a few features that enable other file formats for the config file.

* `json` (default): JSON format
* `toml`: TOML format
* `yaml`: YAML format
* `full`: Enable all formats

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
configura = "^1"
```

JSON format is enabled by default but other format implementations are provided as optional features. To enable YAML instead of JSON (note you can enable all formats but only one can be used for the config file):

```toml
[dependencies]
configura = { version = "^1", default-features = false, features = ["yaml"] }
```

## Usage

```rust
use configura::{Config, formats::JsonFormat, load_config};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    age: u8,
}

impl Config for TestConfig {
    type FormatType = JsonFormat;
    type FormatContext = (); // dont need any context for plain JSON

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (None, "test_config") // `None` uses the home directory and `test_config` is the filename
    }
}

// load the config
let mut config: TestConfig = load_config()?;

// load the config with a default config
let mut config: TestConfig = TestConfig::default();
config.load()?;

// save the config
config.name = "John".into();
config.age = 31;
config.save()?;
```

A mirror/backup file can be used, if provided the data will be written to it after writing to the main file, and when loading if the main file cannot be read the mirror file will be used, this is an example:

```rust
use configura::{Config, formats::JsonFormat, load_config};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
struct TestConfig {
    name: String,
    age: u8,
}

impl Config for TestConfig {
    type FormatType = JsonFormat;
    type FormatContext = (); // dont need any context for plain JSON

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (None, "test_config")
        // `None` uses the home directory so the config path would be in "~/test_config.json" in this case
    }

    fn mirror_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (Some("/tmp/some/other/path".into()), "mirror_config")
        // A path is provided so the mirror config would be in "/tmp/some/other/path/mirror_config.json"
    }
}

// load the config
let mut config: TestConfig = load_config()?;

// load the config with a default config
let mut config: TestConfig = TestConfig::default();
config.load()?;

// save the config
config.name = "John".into();
config.age = 31;
config.save()?;
```

## Tests

Run the tests with `cargo test --all-features`.

## Benchmarks

Run the benchmarks with `cargo bench --all-features`.

* 4.2 GHz AMD Ryzen 7 3800X with 32 GB RAM, Windows 10:

| Benchmark | Time | Outliers |
| --- | --- | --- |
| `Config.load() (no mirror)` | 39.697 µs 39.739 µs 39.786 µs | 2 (2.00%) high mild, 6 (6.00%) high severe |
| `load_config() (no mirror)` | 39.610 µs 39.665 µs 39.737 µs | 7 (7.00%) high mild, 3 (3.00%) high severe |
| `Config.save() (no mirror, overwrite)` | 110.59 µs 110.75 µs 110.92 µs | 5 (5.00%) high mild, 3 (3.00%) high severe |
| `Config.save() (with mirror, overwrite)` | 220.60 µs 220.89 µs 221.26 µs | 4 (4.00%) high mild, 10 (10.00%) high severe |
| `Config.load() (mirror fallback)` | 62.208 µs 62.529 µs 62.959 µs | 4 (4.00%) high mild, 8 (8.00%) high severe |
| `load_config() (mirror fallback)` | 62.196 µs 62.315 µs 62.465 µs | 5 (5.00%) high mild, 7 (7.00%) high severe |
| `Config.load() (mirror, primary exists)` | 39.892 µs 39.945 µs 40.012 µs | 2 (2.00%) high mild, 8 (8.00%) high severe |
| `load_config() (mirror, primary exists)` | 40.052 µs 40.129 µs 40.217 µs | 4 (4.00%) high mild, 4 (4.00%) high severe |

## License

This crate is distributed under the terms of the [MIT license](LICENSE).
