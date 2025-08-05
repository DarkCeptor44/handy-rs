//! # Tempdir
//!
//! Helpers for creating temporary directories with sub-directories and files
//!
//! ## Usage
//!
//! You will need the `tempdir` feature enabled
//!
//! ```rust
//! use handy::helper::TempdirSetupBuilder;
//!
//! // create a temporary directory with 2 files and 10 sub-directories with one 1KB files each
//! let setup = TempdirSetupBuilder::new()
//!     .dir_count(10)
//!     .file_count(15)
//!     .file_size(1024)
//!     .build()
//!     .unwrap();
//! let path = setup.path();
//!
//! // do something with the temporary directory
//! ```
//!
//! The temporary directory will be handled automatically when the [`TempdirSetup`] is dropped
//!
//! ## Benchmarks
//!
//! ```text
//! Timer precision: 100 ns
//! bench_divan                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
//! ╰─ helper                                       │               │               │               │         │
//!    ╰─ tempdir                                   │               │               │               │         │
//!       ├─ pad_content                            │               │               │               │         │
//!       │  ├─ 1024                  199.7 ns      │ 12.49 µs      │ 199.7 ns      │ 354.7 ns      │ 100     │ 100
//!       │  ├─ 1048576               128.6 µs      │ 266.6 µs      │ 183.3 µs      │ 182.5 µs      │ 100     │ 100
//!       │  ╰─ 10485760              1.391 ms      │ 1.689 ms      │ 1.447 ms      │ 1.465 ms      │ 100     │ 100
//!       ╰─ tempdir_setup                          │               │               │               │         │
//!          ├─ (5, 15, 128, 0.15)    4.517 ms      │ 5.705 ms      │ 4.829 ms      │ 4.901 ms      │ 100     │ 100
//!          ├─ (15, 25, 1024, 0.25)  9.336 ms      │ 37.55 ms      │ 9.909 ms      │ 10.47 ms      │ 100     │ 100
//!          ╰─ (50, 50, 4096, 0.5)   19.42 ms      │ 57.64 ms      │ 20.12 ms      │ 21.24 ms      │ 100     │ 100
//! ```

use anyhow::{anyhow, Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::{create_dir, File},
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
    root_file_perc: f64,
}

impl Default for TempdirSetupBuilder {
    fn default() -> Self {
        Self {
            dir_count: 5,
            file_count: 15,
            file_size: 128,
            root_file_perc: 0.15,
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
    /// Default is `5`
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
    /// Default is `15`
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
    /// Default is `128`
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

    /// Sets the arguments to use for `dir_count`, `file_count`, `file_size` and `root_file_perc` in that order. The recommended is to use the individual methods: [`TempdirSetupBuilder::dir_count`], [`TempdirSetupBuilder::file_count`], [`TempdirSetupBuilder::file_size`] and [`TempdirSetupBuilder::root_file_percentage`].
    ///
    /// ## Arguments
    ///
    /// * `args` - A tuple of `(dir_count, file_count, file_size, root_file_perc)`
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().args(&(10, 10, 1024, 0.5)).build().unwrap(); // 10 dirs, 10 files, 1KB, 50% of total files in root
    /// ```
    #[must_use]
    pub fn args(mut self, args: &(usize, usize, u64, f64)) -> Self {
        self.dir_count = args.0;
        self.file_count = args.1;
        self.file_size = args.2;
        self.root_file_perc = args.3;
        self
    }

    /// Sets the percentage of the [total files](TempdirSetup::file_count) that should be created in the root directory. Should be between 0 and 1.
    ///
    /// Default is `0.15`
    ///
    /// ## Arguments
    ///
    /// * `percentage` - The percentage of the total files that should be created in the root directory
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::helper::TempdirSetupBuilder;
    ///
    /// let setup = TempdirSetupBuilder::new().dir_count(100).root_file_percentage(0.5).build().unwrap(); // 50 files in root, 50 divided among sub-directories
    /// ```
    #[must_use]
    pub fn root_file_percentage(mut self, percentage: f64) -> Self {
        self.root_file_perc = percentage;
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
    /// let files = read_dir(setup.path()).unwrap();
    /// ```
    pub fn build(self) -> Result<TempdirSetup> {
        const F64_MAX_EXACT_INT: usize = 1 << f64::MANTISSA_DIGITS;

        let (dirs, files, size, root_perc) = (
            self.dir_count,
            self.file_count,
            self.file_size,
            self.root_file_perc,
        );

        if dirs == 0 || files == 0 || size == 0 {
            return Err(anyhow!(
                "Directory count, file count and file size must be greater than 0"
            ));
        }

        if files > F64_MAX_EXACT_INT {
            return Err(anyhow!("File count must be less than {F64_MAX_EXACT_INT}"));
        }

        if !(0.0..=1.0).contains(&root_perc) {
            return Err(anyhow!("Root file percentage must be between 0 and 1"));
        }

        let temp_dir = tempdir().context("Failed to create temporary directory")?;
        let temp_path = temp_dir.path().to_path_buf();

        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
            clippy::cast_precision_loss
        )]
        let root_files = (self.root_file_perc * files as f64).round() as usize;
        let total_files = files - root_files;
        let files_per_dir = total_files / dirs;

        (0..root_files)
            .par_bridge()
            .map(|i| {
                let file_path = &temp_path.join(format!("file{i}.txt"));
                let mut file = BufWriter::new(
                    File::create(file_path).context("Failed to create temporary file")?,
                );

                file.write_all(&pad_content(size))
                    .context("Failed to write to temporary file")?;
                file.flush().context("Failed to flush temporary file")?;
                drop(file);
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        (0..dirs)
            .par_bridge()
            .map(|i| {
                let dir = &temp_path.join(format!("dir{i}"));
                create_dir(dir).context("Failed to create temporary directory")?;

                for j in 0..files_per_dir {
                    let file_path = &dir.join(format!("file{i}_{j}.txt"));
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
            root_file_percentage: root_perc,
            files_in_root: root_files,
            files_per_subdir: files_per_dir,
            _temp_dir: temp_dir,
            temp_path,
        })
    }
}

/// Helper struct to setup a temporary directory, sub-directories and files for testing and benchmarking
///
/// ## Example
///
/// ```rust,no_run
/// use handy::helper::TempdirSetupBuilder;
///
/// let setup = TempdirSetupBuilder::new()
///     .dir_count(10)
///     .file_count(50)
///     .file_size(1024)
///     .root_file_percentage(0.2)
///     .build()
///     .unwrap();
/// ```
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

    /// The percentage of files to create in the root directory
    pub root_file_percentage: f64,

    /// The number of files to create in the root directory
    pub files_in_root: usize,

    /// The number of files to create in each sub-directory
    pub files_per_subdir: usize,
}

impl TempdirSetup {
    /// Returns the number of entries in the temporary directory
    #[must_use]
    pub fn entries_count(&self) -> usize {
        self.dir_count * self.files_per_subdir + self.dir_count + self.files_in_root
    }

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
    use crate::fs::Walker;

    #[test]
    fn test_tempdir_setup() {
        let setup = TempdirSetupBuilder::new()
            .dir_count(10)
            .file_count(15)
            .file_size(1024)
            .build()
            .expect("Failed to build tempdir setup");

        let entries = Walker::new(setup.path())
            .par_walk()
            .expect("Failed to walk directories");

        assert_eq!(entries.len(), setup.entries_count());
    }

    // TODO add more tests

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
