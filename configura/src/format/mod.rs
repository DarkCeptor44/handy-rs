//! # Format module
//!
//! This module contains pre-defined formats for some common file types

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use json::JsonFormat;
