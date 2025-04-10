use std::path::Path;

use criterion::{black_box, criterion_group, Criterion};
use handy::pattern::{
    glob_to_regex_pattern, is_close_to_upper_bound, match_filename_with_glob_pattern, match_string,
    ERROR_MARGIN,
};

fn bench_pattern(c: &mut Criterion) {
    let mut g = c.benchmark_group("Pattern");

    g.bench_function("glob_to_regex_pattern", |b| {
        b.iter(|| black_box(glob_to_regex_pattern(black_box("fish\\(txt"))));
    });

    g.bench_function("Manual is_close_to_upper_bound", |b| {
        b.iter(|| black_box((0.9999f64 - 1.0).abs() < ERROR_MARGIN));
    });
    g.bench_function("is_close_to_upper_bound", |b| {
        b.iter(|| black_box(is_close_to_upper_bound(black_box(0.9999))));
    });

    g.bench_function("match_filename_with_glob_pattern", |b| {
        b.iter(|| {
            black_box(match_filename_with_glob_pattern(
                black_box(Path::new("fish.txt")),
                black_box("f*.txt"),
            ))
        });
    });

    g.bench_function("match_string", |b| {
        b.iter(|| black_box(match_string(black_box("kitten"), black_box("kissing"))));
    });
}

criterion_group!(pattern, bench_pattern);
