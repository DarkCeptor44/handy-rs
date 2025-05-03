#[cfg(feature = "tempdir")]
pub mod tempdir;

#[cfg(feature = "tempdir")]
pub use tempdir::{pad_content, TempdirSetup, TempdirSetupBuilder};
