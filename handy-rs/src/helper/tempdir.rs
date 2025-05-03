//! Helpers for creating temporary directories with sub-directories and files
//!
//! ## Usage
//!
//! You will need the `tempdir` feature enabled
//!
//! ```rust
//! use handy::helper::TempdirSetupBuilder;
//!
//! // create a temporary directory with 10 sub-directories with 10 1KB files each
//! let setup = TempdirSetupBuilder::new()
//!     .dir_count(10)
//!     .file_count(10)
//!     .file_size(1024)
//!     .build()
//!     .unwrap();
//! let path = setup.path();
//!
//! // do something with the temporary directory
//! ```
//!
//! The temporary directory will be handled automatically when the [`TempdirSetup`] is dropped

use anyhow::{Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use tempfile::{tempdir, TempDir};

/// Builder for [`TempdirSetup`]
#[derive(Debug)]
pub struct TempdirSetupBuilder {
    dir_count: usize,
    file_count: usize,
    file_size: u64,
}

impl Default for TempdirSetupBuilder {
    fn default() -> Self {
        Self {
            dir_count: 5,
            file_count: 5,
            file_size: 128,
        }
    }
}

impl TempdirSetupBuilder {
    /// Creates a new [`TempdirSetupBuilder`] with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of sub-directories to create
    ///
    /// ## Arguments
    ///
    /// * `count` - The number of sub-directories to create
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().dir_count(10).build().unwrap(); // 10 sub-directories
    /// ```
    #[must_use]
    pub fn dir_count(mut self, count: usize) -> Self {
        self.dir_count = count;
        self
    }

    /// Sets the number of files to create in each sub-directory
    ///
    /// ## Arguments
    ///
    /// * `count` - The number of files to create in each sub-directory
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().file_count(10).build().unwrap(); // 10 files
    /// ```
    #[must_use]
    pub fn file_count(mut self, count: usize) -> Self {
        self.file_count = count;
        self
    }

    /// Sets the size in bytes of each file
    ///
    /// ## Arguments
    ///
    /// * `size` - The size in bytes of each file
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().file_size(1048576).build().unwrap(); // 1MB
    /// ```
    #[must_use]
    pub fn file_size(mut self, size: u64) -> Self {
        self.file_size = size;
        self
    }

    /// Sets the arguments to use for `dir_count`, `file_count` and `file_size` in that order. The recommended is to use the individual methods: [`TempdirSetupBuilder::dir_count`], [`TempdirSetupBuilder::file_count`] and [`TempdirSetupBuilder::file_size`].
    ///
    /// ## Arguments
    ///
    /// * `args` - A tuple of `(dir_count, file_count, file_size)`
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().args(&(10, 10, 1024)).build().unwrap(); // 10 dirs, 10 files, 1KB
    /// ```
    #[must_use]
    pub fn args(mut self, args: &(usize, usize, u64)) -> Self {
        self.dir_count = args.0;
        self.file_count = args.1;
        self.file_size = args.2;
        self
    }

    /// Builds the [`TempdirSetup`]
    ///
    /// ## Returns
    ///
    /// A [`TempdirSetup`]
    ///
    /// ## Errors
    ///
    /// Returns an error if the temporary directory or files could not be created/written to.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    /// use std::fs::read_dir;
    ///
    /// let setup = TempdirSetupBuilder::new().build().unwrap();
    ///
    /// let files = read_dir(setup.path()).unwrap();
    /// ```
    pub fn build(self) -> Result<TempdirSetup> {
        let (dirs, files, size) = (self.dir_count, self.file_count, self.file_size);
        let temp_dir = tempdir().context("Failed to create temporary directory")?;
        let temp_path = temp_dir.path().to_path_buf();

        (0..dirs)
            .par_bridge()
            .map(|i| {
                let dir = &temp_path.join(format!("dir{i}"));
                create_dir_all(dir).context("Failed to create temporary directory")?;

                for j in 0..files {
                    let file_path = &dir.join(format!("file{j}.txt"));
                    let mut file = BufWriter::new(
                        File::create(file_path).context("Failed to create temporary file")?,
                    );

                    file.write_all(&pad_content(size))
                        .context("Failed to write to temporary file")?;
                    file.flush().context("Failed to flush temporary file")?;
                    drop(file);
                }

                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(TempdirSetup {
            dir_count: dirs,
            file_count: files,
            file_size: size,
            _temp_dir: temp_dir,
            temp_path,
        })
    }
}

/// Helper struct to setup a temporary directory, sub-directories and files for testing and benchmarking
#[derive(Debug)]
pub struct TempdirSetup {
    _temp_dir: TempDir,
    temp_path: PathBuf,

    /// The number of directories to create
    pub dir_count: usize,

    /// The number of files to create in each directory
    pub file_count: usize,

    /// The size in bytes of each file
    pub file_size: u64,
}

impl TempdirSetup {
    /// Returns the path to the temporary directory
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.temp_path
    }
}

/// Helper function to pad content to a specific length
///
/// ## Arguments
///
/// * `n` - Number of bytes to pad
///
/// ## Returns
///
/// * `Vec<u8>` - Padded content
///
/// ## Panics
///
/// Panics if n is less than or equal to 0 or if failed to convert u64 to usize
///
/// ## Example
///
/// ```rust,no_run
/// use handy::helper::pad_content;
///
/// let content = pad_content(1048576); // 1MB of content
/// ```
#[must_use]
pub fn pad_content(n: u64) -> Vec<u8> {
    const CONTENT: &[u8] = "asdaodkoodoakwodokaokwdokoakowkdokaowkdkoakwodkoakodwkokaodwkooakwodoaokwodkokaowkdkowkodkakowodkoakowkdokoakwdkwkadkwdkoawkdokw".as_bytes();
    assert!(n > 0, "n must be greater than 0");

    CONTENT.repeat(usize::try_from(n).expect("Failed to convert u64 to usize") / CONTENT.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tempdir_setup() {
        let setup = TempdirSetupBuilder::new()
            .dir_count(10)
            .file_count(10)
            .file_size(1024)
            .build()
            .expect("Failed to build tempdir setup");

        // TODO iterate recursively through tempdir
        todo!("iterate recursively through tempdir");
    }

    #[test]
    fn test_pad_content() {
        assert_eq!(pad_content(128).len(), 128);
        assert_eq!(pad_content(1024).len(), 1024);
        assert_eq!(pad_content(1_048_576).len(), 1_048_576);
    }

    #[test]
    #[should_panic(expected = "n must be greater than 0")]
    fn test_pad_content_fail() {
        assert_eq!(pad_content(0).len(), 0);
    }
}
