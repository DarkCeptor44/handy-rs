use divan::black_box;
use handy::pattern;

#[divan::bench]
fn string_similarity() {
    black_box(pattern::string_similarity("kitten", "kissiNg"));
}
