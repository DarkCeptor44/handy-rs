//! # Helper module
//!
//! ## Sub-modules
//!
//! * [`tempdir`] - Helper for creating temporary directories with sub-directories and files

#[cfg(feature = "tempdir")]
pub mod tempdir;

#[cfg(feature = "tempdir")]
pub use tempdir::{pad_content, TempdirSetup, TempdirSetupBuilder};
