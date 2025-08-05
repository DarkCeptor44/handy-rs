use divan::{black_box, Bencher};
use handy::{fs::Walker, helper::TempdirSetupBuilder};
use ignore::WalkBuilder;
use walkdir::WalkDir;

const ARGS: &(usize, usize, u64, f64) = &(10, 10, 1024, 0.25);

#[divan::bench(name = "walker walk")]
fn bench_walker(b: Bencher) {
    let setup = TempdirSetupBuilder::new()
        .args(ARGS)
        .build()
        .expect("Failed to build tempdir setup");

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
    let setup = TempdirSetupBuilder::new()
        .args(ARGS)
        .build()
        .expect("Failed to build tempdir setup");

    b.bench(|| {
        let entries: Vec<_> = Walker::new(setup.path())
            .par_walk()
            .expect("Failed to create walker");
        black_box(entries);
    });
}

#[divan::bench(name = "walkdir")]
fn bench_walkdir(b: Bencher) {
    let setup = TempdirSetupBuilder::new()
        .args(ARGS)
        .build()
        .expect("Failed to build tempdir setup");

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
    let setup = TempdirSetupBuilder::new()
        .args(ARGS)
        .build()
        .expect("Failed to build tempdir setup");

    b.bench(|| {
        let entries: Vec<_> = WalkBuilder::new(setup.path())
            .hidden(false)
            .build()
            .filter_map(std::result::Result::ok)
            .collect();
        black_box(entries);
    });
}
