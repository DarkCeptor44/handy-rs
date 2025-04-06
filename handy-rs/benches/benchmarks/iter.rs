use criterion::{black_box, criterion_group, Criterion};
use handy::iter::{IntoRefVec, StringIterable};

const X: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

fn bench_into_ref_vec(c: &mut Criterion) {
    let mut g = c.benchmark_group("Itertools/IntoRefVec");

    g.bench_function("Manual as_ref_vec", |b| {
        b.iter(|| black_box(X.iter().collect::<Vec<_>>()));
    });
    g.bench_function("as_ref_vec", |b| {
        b.iter(|| black_box(X.as_ref_vec()));
    });

    g.bench_function("Manual as_mut_ref_vec", |b| {
        b.iter(|| {
            let mut x = X;
            black_box(x.iter_mut().collect::<Vec<_>>());
        });
    });
    g.bench_function("as_mut_ref_vec", |b| {
        b.iter(|| {
            let mut x = X;
            black_box(x.as_mut_ref_vec());
        });
    });
}

fn bench_string_iterable(c: &mut Criterion) {
    let mut g = c.benchmark_group("Itertools/StringIterable");

    g.bench_function("Manual to_string_vec", |b| {
        b.iter(|| black_box(X.iter().map(ToString::to_string).collect::<Vec<String>>()));
    });
    g.bench_function("to_string_vec", |b| b.iter(|| black_box(X.to_string_vec())));
}

criterion_group!(iter, bench_into_ref_vec, bench_string_iterable);
