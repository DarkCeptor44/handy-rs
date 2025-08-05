//! # Filesystem utilities
//!
//! This module contains a collection of utilities for working with the filesystem.
//!
//! ## Usage
//!
//! This module requires the `fs` feature to be enabled.
//!
//! ### Walker
//!
//! A [Walker] is a struct that can be used to walk a directory and return the entries, either as an iterator (non-parallel) or as a vector (parallel).
//!
//! ```rust,no_run
//! use handy::fs::Walker;
//!
//! // use walker as an iterator
//! for entry in Walker::new("/path/to/dir").walk().unwrap() {
//!     println!("{}", entry.unwrap().path().display());
//! }
//!
//! // or get entries in parallel as a vector
//! let entries = Walker::new("/path/to/dir").par_walk().unwrap();
//! for entry in entries {
//!     println!("{}", entry.path().display());
//! }
//! ```
//!
//! ## Benchmarks
//!
//! ```text
//! Timer precision: 100 ns
//! bench_divan               fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ╰─ fs                                   │               │               │               │         │
//!    ╰─ dir_walk                          │               │               │               │         │
//!       ├─ ignore           2.292 ms      │ 3.899 ms      │ 2.344 ms      │ 2.439 ms      │ 100     │ 100
//!       ├─ walkdir          489.5 µs      │ 1.126 ms      │ 510.2 µs      │ 526.7 µs      │ 100     │ 100
//!       ├─ walker par_walk  279.8 µs      │ 1.673 ms      │ 344 µs        │ 378.8 µs      │ 100     │ 100
//!       ╰─ walker walk      2.363 ms      │ 3.873 ms      │ 2.437 ms      │ 2.58 ms       │ 100     │ 100
//! ```

use crate::errors::FsError;
use anyhow::Result;
use colored::Colorize;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::{read_dir, DirEntry, ReadDir},
    path::{Path, PathBuf},
};
use tempfile::{tempdir, TempDir};

/// A directory walker meant to be faster than alternatives like [`walkdir`](https://crates.io/crates/walkdir) and [`ignore`](https://crates.io/crates/ignore) but still close to [`std::fs::read_dir`], returning [`std::fs::DirEntry`] instead of a custom wrapper.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::fs::Walker;
///
/// // use walker as an iterator
/// for entry in Walker::new("/path/to/dir").walk().unwrap() {
///     println!("{}", entry.unwrap().path().display());
/// }
/// ```
///
/// ```rust,no_run
/// use handy::fs::Walker;
///
/// // use walker as a vector
/// let entries = Walker::new("/path/to/dir").par_walk().unwrap();
/// for entry in entries {
///     println!("{}", entry.path().display());
/// }
/// ```
#[derive(Debug)]
pub struct Walker {
    current: Option<ReadDir>,
    to_walk: Vec<PathBuf>,

    path: PathBuf,
    colored: bool,
    print: bool,
}

impl Walker {
    /// Create a new [Walker]
    ///
    /// ## Arguments
    ///
    /// * `path` - The path to walk
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::fs::Walker;
    ///
    /// let walker = Walker::new("/path/to/dir");
    /// ```
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        Self {
            current: None,
            to_walk: Vec::new(),
            path: path.to_path_buf(),
            colored: false,
            print: false,
        }
    }

    /// Set whether or not to color the output of the printing. This mostly applies to [`Walker::par_walk`]
    ///
    /// Default: `false`
    ///
    /// ## Arguments
    ///
    /// * `colored` - Whether or not to color the output
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::fs::Walker;
    ///
    /// let walker = Walker::new("/path/to/dir").colored(true);
    /// ```
    #[must_use]
    pub fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }

    /// Set whether or not to print errors during walking. This mostly applies to [`Walker::par_walk`]
    ///
    /// Default: `false`
    ///
    /// ## Arguments
    ///
    /// * `print` - Whether or not to print the errors or warnings
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::fs::Walker;
    ///
    /// let walker = Walker::new("/path/to/dir").print(true);
    /// ```
    #[must_use]
    pub fn print(mut self, print: bool) -> Self {
        self.print = print;
        self
    }

    /// Print an error message
    fn eprintln(&self, err: &FsError) {
        if self.print {
            if self.colored {
                eprintln!("{}", err.to_string().red());
            } else {
                eprintln!("{err}");
            }
        }
    }

    /// Start walking the directory
    ///
    /// ## Returns
    ///
    /// Returns a [Walker] which can be used as an iterator
    ///
    /// ## Errors
    ///
    /// Returns an error if the path does not exist or if the entries could not be read
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::fs::Walker;
    ///
    /// for entry in Walker::new("/path/to/dir").walk().unwrap() {
    ///     println!("{}", entry.unwrap().path().display());
    /// }
    /// ```
    pub fn walk(mut self) -> std::io::Result<Self> {
        self.current = Some(read_dir(&self.path)?);
        Ok(self)
    }

    /// Start walking the directory in parallel
    ///
    /// ## Returns
    ///
    /// Returns a vector of [`DirEntry`]
    ///
    /// ## Errors
    ///
    /// Returns an error if the path does not exist or if the entries could not be read
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::fs::Walker;
    ///
    /// for entry in Walker::new("/path/to/dir").par_walk().unwrap() {
    ///     println!("{}", entry.path().display());
    /// }
    /// ```
    pub fn par_walk(&self) -> Result<Vec<DirEntry>> {
        let path = &self.path;

        if !path.exists() {
            return Err(FsError::path_does_not_exist(path).into());
        }

        if !path.is_dir() {
            return Err(FsError::path_is_not_directory(path).into());
        }

        self.par_walk_inner(path)
    }

    /// Start walking the directory in parallel
    fn par_walk_inner<P>(&self, path: P) -> Result<Vec<DirEntry>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let entries: Vec<DirEntry> = if let Ok(entries) = read_dir(path) {
            entries
                .filter_map(|e| {
                    e.inspect_err(|_| {
                        self.eprintln(&FsError::DirEntry);
                    })
                    .ok()
                })
                .collect()
        } else {
            self.eprintln(&FsError::dir_read(path));
            return Ok(vec![]);
        };

        let results: Vec<Result<Vec<DirEntry>>> = entries
            .into_par_iter()
            .map(|e| {
                let entry_path = e.path();
                let Ok(file_type) = e.file_type() else {
                    self.eprintln(&FsError::FileType(entry_path));
                    return Ok(vec![]);
                };

                if file_type.is_file() {
                    Ok(vec![e])
                } else if file_type.is_dir() {
                    let mut entries = vec![e];
                    entries.extend(self.par_walk_inner(entry_path)?);
                    Ok(entries)
                } else {
                    self.eprintln(&FsError::NonFileNonDir(entry_path));
                    Ok(vec![])
                }
            })
            .collect();

        let mut all_entries = Vec::new();
        for result in results {
            all_entries.extend(result?);
        }

        Ok(all_entries)
    }
}

impl Iterator for Walker {
    type Item = std::io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut current_iter) = self.current {
                match current_iter.next() {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if path.is_dir() {
                            self.to_walk.push(path);
                        }
                        return Some(Ok(entry));
                    }
                    Some(Err(e)) => {
                        return Some(Err(e));
                    }
                    None => {
                        self.current = None;
                    }
                }
            }

            if let Some(next_dir_path) = self.to_walk.pop() {
                match read_dir(next_dir_path) {
                    Ok(new_iter) => {
                        self.current = Some(new_iter);
                    }
                    Err(e) => {
                        return Some(Err(e));
                    }
                }
            } else {
                return None;
            }
        }
    }
}

// TODO replace with TempdirSetup
struct Bench {
    _temp_dir: TempDir,
    temp_path: PathBuf,
}

impl Bench {
    fn new() -> Self {
        let dir = tempdir().expect("Failed to create temp dir");
        let dir_path = dir.path().to_path_buf();

        for i in 0..15 {
            let file_path = dir_path.join(format!("file_{i}.txt"));
            std::fs::write(file_path, b"hello").expect("Failed to write file");
        }

        for i in 0..10 {
            let dir_path2 = dir_path.join(format!("dir_{i}"));
            std::fs::create_dir(&dir_path2).expect("Failed to create dir");

            for j in 0..5 {
                let file_path = dir_path2.join(format!("file_{i}{j}.txt"));
                std::fs::write(file_path, b"hello").expect("Failed to write file");
            }
        }

        Bench {
            _temp_dir: dir,
            temp_path: dir_path,
        }
    }

    fn path(&self) -> &Path {
        &self.temp_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walker_iter() {
        // TODO use TempdirSetup
        let setup = Bench::new();
        let walker = Walker::new(setup.path())
            .walk()
            .expect("Failed to create walker");
        assert_eq!(dbg!(walker).count(), 75);
    }

    #[test]
    fn test_walker_parallel() {
        // TODO use TempdirSetup
        let setup = Bench::new();
        let entries = Walker::new(setup.path())
            .par_walk()
            .expect("Failed to create walker");
        assert_eq!(dbg!(entries).len(), 75);
    }
}
