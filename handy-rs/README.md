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
handy-rs = "^2"

# to enable all features
handy-rs = { version = "^2", features = ["full"] }
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
| `Formatting/human_bytes`            | 165.17 ns 165.68 ns 166.29 ns              | 1 (1.00%) high mild, 6 (6.00%) high severe |
| `Formatting/human_bytes_as_parts`   | 6.7940 ns 6.8440 ns 6.9058 ns              | 5 (5.00%) high mild, 9 (9.00%) high severe                   |
| `Formatting/human_bytes_si`         | 163.04 ns 163.70 ns 164.70 ns              | 6 (6.00%) high mild, 2 (2.00%) high severe                   |
| `Formatting/human_bytes_si_as_parts`| 6.7010 ns 6.7192 ns 6.7421 ns              | 1 (1.00%) high mild, 8 (8.00%) high severe                  |
| `Formatting/human_number`           | 161.62 ns 162.11 ns 162.69 ns              | 5 (5.00%) high mild, 5 (5.00%) high severe                   |
| `Formatting/human_number_as_parts`  | 6.7572 ns 6.7797 ns 6.8068 ns              | 3 (3.00%) low mild, 2 (2.00%) high mild, 9 (9.00%) high severe                   |
| `Formatting/Humanizer::format`      | 162.80 ns 163.22 ns 163.68 ns              | 3 (3.00%) high mild, 4 (4.00%) high severe                   |
| `Formatting/Humanizer::format_as_parts` | 5.3102 ns 5.3168 ns 5.3254 ns          | 4 (4.00%) high mild, 8 (8.00%) high severe                   |
| `Parse/split_at_non_digits`         | 63.391 ns 63.450 ns 63.520 ns              | 1 (1.00%) low severe, 1 (1.00%) low mild, 4 (4.00%) high mild, 5 (5.00%) high severe |

## License

This crate is distributed under the terms of the [MIT license](LICENSE).
