use configura::{errors::Result, formats::JsonFormat, load_config, Config};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs::remove_file, path::PathBuf};
use tempfile::tempdir;

const BENCH_FILENAME: &str = "test_config_bench";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct TestConfig {
    name: String,
    age: u8,
}

impl Config for TestConfig {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(_: &std::path::Path) -> (Option<PathBuf>, &str) {
        (None, BENCH_FILENAME)
    }
}

fn setup_config(data: &TestConfig) -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path().display().to_string();

    temp_env::with_vars(
        vec![
            ("HOME", Some(temp_path.clone())),
            #[cfg(windows)]
            ("USERPROFILE", Some(temp_path)),
        ],
        || data.save(),
    )?;
    Ok(temp_dir.into_path().join("test_config_bench.json"))
}

fn post_bench_cleanup() {
    let home = home_dir().unwrap();
    let path = home.join(format!("{BENCH_FILENAME}.json"));

    remove_file(path).unwrap();
}

fn bench_config_load(c: &mut Criterion) {
    let config_data = TestConfig {
        name: "Benchmark user".into(),
        age: 99,
    };
    setup_config(&config_data).unwrap();

    c.bench_function("Config.load()", |b| {
        b.iter(|| {
            let mut loaded_config = TestConfig::default();
            loaded_config.load().unwrap();
            black_box(loaded_config);
        });
    });

    c.bench_function("load_config()", |b| {
        b.iter(|| {
            let loaded_config: TestConfig = load_config().unwrap();
            black_box(loaded_config);
        });
    });

    post_bench_cleanup();
}

fn bench_config_save(c: &mut Criterion) {
    let config_data = TestConfig {
        name: "Benchmark user".into(),
        age: 99,
    };
    setup_config(&config_data).unwrap();

    c.bench_function("Config.save()", |b| {
        b.iter(|| {
            let to_save = config_data.clone();
            to_save.save().unwrap();
            black_box(to_save);
        });
    });

    post_bench_cleanup();
}

criterion_group!(benches, bench_config_load, bench_config_save);
criterion_main!(benches);
