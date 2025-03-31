# handy-rs

A collection of often used logic in my Rust projects.

## Features

Each feature enables a module with the same name containing the logic.

* [`collections`](./src/collections.rs): Concurrent collections like `ConcurrentHashMap` and `ConcurrentBTreeMap`.
* [`human`](./src/human.rs): Human readable formatting of numbers and bytes.
* [`parse`](./src/parse.rs): Parsing of numbers and strings.
* [`pattern`](./src/pattern.rs): Glob pattern matching.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
handy-rs = "^1"

# to enable all features
handy-rs = { version = "^1", features = ["full"] }
```

## Usage

Refer to the documentation for each module.

## Tests

Run the tests with `cargo test`.

## Benchmarks

Run the benchmarks with `cargo bench`.

* 4.2 GHz AMD Ryzen 7 3800X with 32 GB RAM, Windows 10:

| Benchmark                             | Time                                       | Outliers                                                     |
| :------------------------------------ | :----------------------------------------- | :----------------------------------------------------------- |
| `HashMap/insert`                      | 101.53 ns 102.29 ns 103.27 ns              | 1 (1.00%) high mild                                        |
| `HashMap/get`                       | 13.519 ms 13.568 ms 13.642 ms              | 2 (2.00%) high mild, 6 (6.00%) high severe                   |
| `BTreeMap/insert`                   | 168.49 ns 169.27 ns 170.18 ns              | 4 (4.00%) high mild, 6 (6.00%) high severe                   |
| `BTreeMap/get`                      | 50.700 ms 50.831 ms 50.976 ms              | 1 (1.00%) high mild, 1 (1.00%) high severe                   |
| `ConcurrentHashMap/sequential insert` | 96.134 ns 98.969 ns 101.85 ns              | 1 (1.00%) low mild, 3 (3.00%) high mild                    |
| `ConcurrentHashMap/concurrent insert` | 3.0921 ms 3.1072 ms 3.1233 ms              | 4 (4.00%) high mild                                        |
| `ConcurrentBTreeMap/sequential insert`| 131.69 ns 132.64 ns 133.51 ns              | 4 (4.00%) low mild                                         |
| `ConcurrentBTreeMap/concurrent insert`| 3.6041 ms 3.6127 ms 3.6215 ms              | 2 (2.00%) high mild                                        |
| `Formatting/human_bytes`            | 145.02 ns 145.60 ns 146.41 ns              | 1 (1.00%) low mild, 4 (4.00%) high mild, 4 (4.00%) high severe |
| `Formatting/human_bytes_as_parts`   | 51.737 ns 52.153 ns 52.637 ns              | 2 (2.00%) high mild, 1 (1.00%) high severe                   |
| `Formatting/human_bytes_si`         | 155.73 ns 156.00 ns 156.28 ns              | 8 (8.00%) high mild, 1 (1.00%) high severe                   |
| `Formatting/human_bytes_si_as_parts`| 56.630 ns 56.741 ns 56.894 ns              | 3 (3.00%) high mild, 10 (10.00%) high severe                  |
| `Formatting/human_number`           | 207.89 ns 208.25 ns 208.71 ns              | 3 (3.00%) high mild, 2 (2.00%) high severe                   |
| `Formatting/human_number_as_parts`  | 55.319 ns 55.478 ns 55.679 ns              | 5 (5.00%) high mild, 8 (8.00%) high severe                   |
| `Parse/split_at_non_digits`         | 63.391 ns 63.450 ns 63.520 ns              | 1 (1.00%) low severe, 1 (1.00%) low mild, 4 (4.00%) high mild, 5 (5.00%) high severe |

## License

This crate is distributed under the terms of the [MIT license](LICENSE).
