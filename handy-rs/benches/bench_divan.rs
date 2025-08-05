#[cfg(feature = "fs")]
mod fs;

mod helper;

#[cfg(feature = "pattern")]
mod pattern;

fn main() {
    divan::main();
}
