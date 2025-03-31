use criterion::{black_box, criterion_group, Criterion};
use handy::human::{
    human_bytes, human_bytes_as_parts, human_bytes_si, human_bytes_si_as_parts, human_number,
    human_number_as_parts,
};

const X: u64 = 123_456_789_012_345;

fn bench_human(c: &mut Criterion) {
    let mut g = c.benchmark_group("Formatting");

    g.bench_function("human_bytes", |b| {
        b.iter(|| black_box(human_bytes(black_box(X))))
    });
    g.bench_function("human_bytes_as_parts", |b| {
        b.iter(|| black_box(human_bytes_as_parts(black_box(X))))
    });

    g.bench_function("human_bytes_si", |b| {
        b.iter(|| black_box(human_bytes_si(black_box(X))))
    });
    g.bench_function("human_bytes_si_as_parts", |b| {
        b.iter(|| black_box(human_bytes_si_as_parts(black_box(X))))
    });

    g.bench_function("human_number", |b| {
        b.iter(|| black_box(human_number(black_box(X))))
    });
    g.bench_function("human_number_as_parts", |b| {
        b.iter(|| black_box(human_number_as_parts(black_box(X))))
    });
}

criterion_group!(human, bench_human);
