use divan::black_box;
use handy::helper::{pad_content, TempdirSetupBuilder};

#[divan::bench(name = "tempdir_setup", args = [(5, 15, 128, 0.15), (15, 25, 1024, 0.25), (50, 50, 4096, 0.5)])]
fn bench_tempdir_setup(args: &(usize, usize, u64, f64)) {
    black_box(
        TempdirSetupBuilder::new()
            .args(args)
            .build()
            .expect("Failed to build tempdir setup"),
    );
}

#[divan::bench(name = "pad_content", args = [1024, 1_048_576, 10_485_760])]
fn bench_pad_content(n: u64) {
    black_box(pad_content(n));
}
