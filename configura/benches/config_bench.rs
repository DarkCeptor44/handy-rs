use configura::{formats::JsonFormat, load_config, Config};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{self, remove_file},
    path::{Path, PathBuf},
};
use tempfile::TempDir;

const BENCH_FILENAME: &str = "test_config_bench";
const BENCH_FILENAME_MIRROR: &str = "test_config_bench_mirror";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct TestConfig {
    name: String,
    age: u8,
}

impl Config for TestConfig {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(home_dir: &Path) -> (Option<PathBuf>, &str) {
        (Some(home_dir.to_path_buf()), BENCH_FILENAME)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct TestConfigWithMirror {
    name: String,
    age: u8,
}

impl Config for TestConfigWithMirror {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(home_dir: &Path) -> (Option<PathBuf>, &str) {
        (Some(home_dir.to_path_buf()), BENCH_FILENAME)
    }

    fn mirror_path_and_filename(home_dir: &Path) -> (Option<PathBuf>, &str) {
        (Some(home_dir.to_path_buf()), BENCH_FILENAME_MIRROR)
    }
}

fn run_in_temp_home<F>(test_fn: F)
where
    F: FnOnce(&Path),
{
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path_str = temp_dir
        .path()
        .to_str()
        .expect("Temp path not valid UTF-8")
        .to_string();

    temp_env::with_vars(
        vec![
            ("HOME", Some(OsStr::new(&temp_path_str))),
            #[cfg(windows)]
            ("USERPROFILE", Some(OsStr::new(&temp_path_str))),
        ],
        || {
            test_fn(temp_dir.path());
        },
    );
}

fn bench_config_load_no_mirror(c: &mut Criterion) {
    let config_data = TestConfig {
        name: "Benchmark user".into(),
        age: 99,
    };

    run_in_temp_home(|_temp_path| {
        config_data
            .save()
            .expect("Setup: Failed to save initial config");

        c.bench_function("Config.load() (no mirror)", |b| {
            b.iter(|| {
                let mut loaded_config = TestConfig::default();
                loaded_config.load().unwrap();
                black_box(loaded_config);
            });
        });

        c.bench_function("load_config() (no mirror)", |b| {
            b.iter(|| {
                let loaded_config: TestConfig = load_config().unwrap();
                black_box(loaded_config);
            });
        });
    });
}

fn bench_config_save_no_mirror(c: &mut Criterion) {
    let config_data = TestConfig {
        name: "Benchmark user".into(),
        age: 99,
    };

    run_in_temp_home(|_temp_path| {
        config_data
            .save()
            .expect("Setup: Failed to save initial config");

        c.bench_function("Config.save() (no mirror, overwrite)", |b| {
            b.iter(|| {
                let to_save = config_data.clone();
                to_save.save().unwrap();
                black_box(to_save);
            });
        });
    });
}

fn bench_config_save_with_mirror(c: &mut Criterion) {
    let config_data = TestConfigWithMirror {
        name: "Benchmark user mirror".into(),
        age: 101,
    };

    run_in_temp_home(|_temp_path| {
        config_data
            .save()
            .expect("Setup: Failed to save initial config");

        c.bench_function("Config.save() (with mirror, overwrite)", |b| {
            b.iter(|| {
                let to_save = config_data.clone();
                to_save.save().unwrap();
                black_box(to_save);
            });
        });
    });
}

fn bench_config_load_mirror_fallback(c: &mut Criterion) {
    let config_data = TestConfigWithMirror {
        name: "Benchmark user mirror".into(),
        age: 101,
    };

    run_in_temp_home(|_temp_path| {
        let primary_path = config_data.path().unwrap();
        let mirror_path = config_data.get_mirror_path().unwrap().unwrap(); // We know it's Some
        let data_str = config_data.to_string(false).unwrap();
        fs::write(&mirror_path, data_str).expect("Setup: Failed to write mirror file");

        if primary_path.exists() {
            remove_file(&primary_path).unwrap();
        }

        c.bench_function("Config.load() (mirror fallback)", |b| {
            b.iter(|| {
                let mut loaded_config = TestConfigWithMirror::default();
                loaded_config.load().unwrap();
                black_box(loaded_config);
            });
        });

        c.bench_function("load_config() (mirror fallback)", |b| {
            b.iter(|| {
                let loaded_config: TestConfigWithMirror = load_config().unwrap();
                black_box(loaded_config);
            });
        });
    });
}

fn bench_config_load_mirror_primary_exists(c: &mut Criterion) {
    let config_data = TestConfigWithMirror {
        name: "Benchmark user mirror".into(),
        age: 101,
    };

    run_in_temp_home(|_temp_path| {
        config_data
            .save()
            .expect("Setup: Failed to save initial config");

        c.bench_function("Config.load() (mirror, primary exists)", |b| {
            b.iter(|| {
                let mut loaded_config = TestConfigWithMirror::default();
                loaded_config.load().unwrap();
                black_box(loaded_config);
            });
        });

        c.bench_function("load_config() (mirror, primary exists)", |b| {
            b.iter(|| {
                let loaded_config: TestConfigWithMirror = load_config().unwrap();
                black_box(loaded_config);
            });
        });
    });
}

criterion_group!(
    benches,
    bench_config_load_no_mirror,
    bench_config_save_no_mirror,
    bench_config_save_with_mirror,
    bench_config_load_mirror_fallback,
    bench_config_load_mirror_primary_exists
);
criterion_main!(benches);
