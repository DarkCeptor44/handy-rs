use criterion::{black_box, criterion_group, Criterion};
use handy::parse::split_at_non_digits;

fn bench_parse(c: &mut Criterion) {
    let mut g = c.benchmark_group("Parse");

    g.bench_function("split_at_non_digits", |b| {
        b.iter(|| black_box(split_at_non_digits::<u32>(black_box("123ab4c")).unwrap()))
    });
}

criterion_group!(parse, bench_parse);
