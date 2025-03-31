# configura

Convenient trait-based configuration loading and saving logic. This allows you to add configuration files to any Rust project, providing the file name, the format (which is JSON by default so the extension is `.json`), and the path to the folder where the config file will be stored (which if not specified defaults to the home directory of the machine).

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

## Tests

Run the tests with `cargo test`.

## Benchmarks

Run the benchmarks with `cargo bench`.

* 4.2 GHz AMD Ryzen 7 3800X with 32 GB RAM, Windows 10:

| Benchmark | Time | Outliers |
| --- | --- | --- |
| `Config.load()` | 40.576 μs 40.635 μs 40.704 μs | 6 (6.00%) high mild, 6 (6.00%) high severe |
| `load_config()` | 40.695 µs 40.762 µs 40.847 µs | 4 (4.00%) high mild, 1 (1.00%) high severe |
| `Config.save()` | 111.85 µs 112.15 µs 112.61 µs | 5 (5.00%) high mild, 7 (7.00%) high severe |

## License

This crate is distributed under the terms of the [MIT license](LICENSE).
