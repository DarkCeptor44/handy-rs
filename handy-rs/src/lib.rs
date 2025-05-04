#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub mod errors;

#[cfg(feature = "collections")]
pub mod collections;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "human")]
pub mod human;

#[cfg(feature = "itertools")]
pub mod iter;

#[cfg(feature = "parse")]
pub mod parse;

#[cfg(feature = "pattern")]
pub mod pattern;
