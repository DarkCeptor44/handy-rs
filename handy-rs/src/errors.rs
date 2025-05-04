use std::path::{Path, PathBuf};
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

/// Errors that can occur when working with the [filesystem](`crate::fs`) module.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FsError {
    #[error("path does not exist: {0}")]
    PathDoesNotExist(PathBuf),

    #[error("path is not a directory: {0}")]
    PathIsNotDirectory(PathBuf),

    #[error("failed to read directory entry")]
    DirEntry,

    #[error("failed to read directory: {0}")]
    DirRead(PathBuf),

    #[error("failed to get file type for `{0}`")]
    FileType(PathBuf),

    #[error("skipping non-file/non-directory entry: {0}")]
    NonFileNonDir(PathBuf),
}

impl FsError {
    /// Create a new [`FsError::PathDoesNotExist`]
    pub fn path_does_not_exist<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::PathDoesNotExist(path.as_ref().to_path_buf())
    }

    /// Create a new [`FsError::PathIsNotDirectory`]
    pub fn path_is_not_directory<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::PathIsNotDirectory(path.as_ref().to_path_buf())
    }

    /// Create a new [`FsError::DirRead`]
    pub fn dir_read<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::DirRead(path.as_ref().to_path_buf())
    }

    /// Create a new [`FsError::FileType`]
    pub fn file_type<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::FileType(path.as_ref().to_path_buf())
    }

    /// Create a new [`FsError::NonFileNonDir`]
    pub fn non_file_non_dir<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::NonFileNonDir(path.as_ref().to_path_buf())
    }
}
