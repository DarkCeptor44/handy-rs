use thiserror::Error;

/// Errors that can occur when using the concurrent collections from the collections module.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConcurrentCollectionError {
    #[error("lock is poisoned")]
    Poison,
}

/// Errors that can occur when parsing numbers.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("parse error: invalid number: {0}")]
    InvalidNumber(String),
}
