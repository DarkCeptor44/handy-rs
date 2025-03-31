use criterion::{BenchmarkGroup, Criterion, black_box, criterion_group, measurement::WallTime};
use rand::Rng;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{BTreeMap, HashMap};
use handy::collections::{ConcurrentBTreeMap, ConcurrentHashMap, Map};

fn bench_hash_map(c: &mut Criterion) {
    let mut g = c.benchmark_group("HashMap");

    let mut rng = rand::rng();
    let mut map: HashMap<usize, u32> = HashMap::new();

    g.bench_function("insert", |b| {
        b.iter(|| {
            let key = rng.random_range(0..1_000_000);
            let value = rng.random_range(0..1_000_000);
            map.insert(black_box(key), black_box(value));
        })
    });

    let keys_to_lookup: Vec<usize> = map.keys().copied().collect();
    g.bench_function("get", |b| {
        b.iter(|| {
            for &key in &keys_to_lookup {
                black_box(map.get(black_box(&key)));
            }
        });
    });
}

fn bench_btree_map(c: &mut Criterion) {
    let mut g = c.benchmark_group("BTreeMap");

    let mut rng = rand::rng();
    let mut map: BTreeMap<usize, u32> = BTreeMap::new();

    g.bench_function("insert", |b| {
        b.iter(|| {
            let key = rng.random_range(0..1_000_000);
            let value = rng.random_range(0..1_000_000);
            map.insert(black_box(key), black_box(value));
        })
    });

    let keys_to_lookup: Vec<usize> = map.keys().copied().collect();
    g.bench_function("get", |b| {
        b.iter(|| {
            for &key in &keys_to_lookup {
                black_box(map.get(black_box(&key)));
            }
        });
    });
}

fn bench_concurrent_hash_map(c: &mut Criterion) {
    let mut g = c.benchmark_group("ConcurrentHashMap");

    bench_map::<ConcurrentHashMap<_, _>>(&mut g);
}

fn bench_concurrent_btree_map(c: &mut Criterion) {
    let mut g = c.benchmark_group("ConcurrentBTreeMap");

    bench_map::<ConcurrentBTreeMap<_, _>>(&mut g);
}

fn bench_map<M>(g: &mut BenchmarkGroup<'_, WallTime>)
where
    M: Map<usize, u32> + Send + Sync,
{
    g.bench_function("sequential insert", |b| {
        let mut rng = rand::rng();
        let map = M::new();

        b.iter(|| {
            let key = rng.random_range(0..1_000_000);
            let value = rng.random_range(0..1_000_000);
            map.insert(black_box(key), black_box(value)).unwrap();
        });
    });

    g.bench_function("concurrent insert", |b| {
        let map = M::new();
        let num_threads = 8;
        let ops_per_thread = 1000;
        let total_ops = num_threads * ops_per_thread;

        b.iter(|| {
            (0..total_ops).par_bridge().for_each(|_| {
                let mut rng = rand::rng();
                let key = rng.random_range(0..1_000_000);
                let value = rng.random_range(0..1_000_000);
                map.insert(black_box(key), black_box(value)).unwrap();
            })
        });
    });
}

criterion_group!(
    collections,
    bench_hash_map,
    bench_btree_map,
    bench_concurrent_hash_map,
    bench_concurrent_btree_map
);
