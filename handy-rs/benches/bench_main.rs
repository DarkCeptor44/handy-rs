use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::collections::collections,
    benchmarks::human::human,
    benchmarks::parse::parse
);
