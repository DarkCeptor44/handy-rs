use thiserror::Error;

pub type Result<T> = core::result::Result<T, TableError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TableError {
    #[error("header length ({0}) must match row length ({1})")]
    HeaderLengthMismatch(usize, usize),

    #[error("row length ({0}) must match first row length ({1})")]
    RowLengthMismatch(usize, usize),
}
