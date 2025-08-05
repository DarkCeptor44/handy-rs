use divan::black_box;
use handy::pattern::{self, ERROR_MARGIN};
use std::path::Path;

#[divan::bench]
fn glob_to_regex_pattern() {
    black_box(pattern::glob_to_regex_pattern(black_box("fish\\(txt")));
}

#[divan::bench]
fn is_close_to_upper_bound() {
    black_box(pattern::is_close_to_upper_bound(black_box(0.9999)));
}

#[divan::bench(name = "is_close_to_upper_bound (Manual)")]
fn is_close_to_upper_bound_manual() {
    black_box((0.9999f64 - 1.0).abs() < ERROR_MARGIN);
}

#[divan::bench]
fn match_filename_with_glob_pattern() {
    black_box(pattern::match_filename_with_glob_pattern(
        black_box(Path::new("fish.txt")),
        black_box("f*.txt"),
    ));
}

#[divan::bench]
fn match_string() {
    black_box(pattern::match_string(
        black_box("kitten"),
        black_box("kissIng"),
    ));
}

#[divan::bench]
fn string_similarity() {
    black_box(pattern::string_similarity(
        black_box("kitten"),
        black_box("kissIng"),
    ));
}
