use thiserror::Error;

pub type Result<T> = core::result::Result<T, TableError>;

/// A enum that represents an error that can occur while using the [Table](crate::Table) struct.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TableError {
    /// Header length does not match row length
    #[error("header length ({0}) must match row length ({1})")]
    HeaderLengthMismatch(usize, usize),

    /// Row length does not match first row length
    #[error("row length ({0}) must match first row length ({1})")]
    RowLengthMismatch(usize, usize),
}
