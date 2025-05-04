use divan::{black_box, Bencher};
use handy::fs::Walker;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

// TODO use TempdirSetup

#[divan::bench(name = "walker walk")]
fn bench_walker(b: Bencher) {
    let setup = Bench::new();

    b.bench(|| {
        let entries: Vec<_> = Walker::new(setup.path())
            .walk()
            .expect("Failed to create walker")
            .filter_map(std::result::Result::ok)
            .collect();
        black_box(entries);
    });
}

#[divan::bench(name = "walker par_walk")]
fn bench_walker_parallel(b: Bencher) {
    let setup = Bench::new();

    b.bench(|| {
        let entries: Vec<_> = Walker::new(setup.path())
            .par_walk()
            .expect("Failed to create walker");
        black_box(entries);
    });
}

#[divan::bench(name = "walkdir")]
fn bench_walkdir(b: Bencher) {
    let setup = Bench::new();

    b.bench(|| {
        let entries: Vec<_> = WalkDir::new(setup.path())
            .into_iter()
            .filter_map(std::result::Result::ok)
            .collect();
        black_box(entries);
    });
}

#[divan::bench(name = "ignore")]
fn bench_ignore(b: Bencher) {
    let setup = Bench::new();

    b.bench(|| {
        let entries: Vec<_> = WalkBuilder::new(setup.path())
            .hidden(false)
            .build()
            .filter_map(std::result::Result::ok)
            .collect();
        black_box(entries);
    });
}

// swap with TempdirSetup
struct Bench {
    _temp_dir: TempDir,
    temp_path: PathBuf,
}

impl Bench {
    fn new() -> Self {
        let dir = tempdir().expect("Failed to create temp dir");
        let dir_path = dir.path().to_path_buf();

        for i in 0..15 {
            let file_path = dir_path.join(format!("file_{i}.txt"));
            std::fs::write(file_path, b"hello").expect("Failed to write file");
        }

        for i in 0..10 {
            let dir_path2 = dir_path.join(format!("dir_{i}"));
            std::fs::create_dir(&dir_path2).expect("Failed to create dir");

            for j in 0..5 {
                let file_path = dir_path2.join(format!("file_{i}{j}.txt"));
                std::fs::write(file_path, b"hello").expect("Failed to write file");
            }
        }

        Bench {
            _temp_dir: dir,
            temp_path: dir_path,
        }
    }

    fn path(&self) -> &Path {
        &self.temp_path
    }
}
