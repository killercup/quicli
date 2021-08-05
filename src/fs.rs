//! Some helpful functions to deal with file system operations.
//!
//! These are rather simple, but provide a quick and easy way to to common
//! tasks. Also, they have great error messages.

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Read, Write};
use std::fs::File;
use std::result::Result as StdResult;

use failure::{Error, ResultExt, ensure};

pub use std::fs::create_dir_all as create_dir;
pub use remove_dir_all::remove_dir_all;

/// Read file content into string
///
/// # Examples
///
/// ```rust
/// # use quicli::prelude::*;
/// # fn main() -> CliResult {
/// let x = read_file(".gitignore")?;
/// assert!(x.len() > 0);
/// # Ok(()) }
/// ```
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    ensure!(
        path.exists() && path.is_file(),
        "Path {:?} is not a file!",
        path
    );

    let file = File::open(path).with_context(|_| format!("Could not open file {:?}", path))?;
    let mut file = BufReader::new(file);

    let mut result = String::new();
    file.read_to_string(&mut result)
        .with_context(|_| format!("Could not read file {:?}", path))?;

    Ok(result)
}

/// Write string to file
///
/// _Note:_ Replaces the current file content if the file already exists.
///
/// # Examples
///
/// ```rust
/// # use quicli::prelude::*;
/// # fn main() -> CliResult {
/// let dir = tempfile::tempdir()?;
/// let filepath = dir.path().join("asdasidz81zasda");
/// write_to_file(filepath, "foobar")?;
/// # Ok(()) }
/// ```
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
    let path = path.as_ref();

    let file =
        File::create(path).with_context(|_| format!("Could not create/open file {:?}", path))?;
    let mut file = BufWriter::new(file);

    file.write_all(content.as_bytes())
        .with_context(|_| format!("Could not write to file {:?}", path))?;

    Ok(())
}

/// Find files with glob pattern
///
/// Search for a pattern like `*.md` and get an iterator of Markdown files.
///
/// # Examples
///
/// ```rust
/// # use quicli::prelude::*;
/// # fn main() -> CliResult {
/// let markdown_files = glob("*.md")?;
/// assert_eq!(markdown_files.len(), 2);
///
/// let image_files = glob("**/*.{png,jpg,gif}");
/// assert!(image_files.is_err());
/// if let Err(e) = image_files {
///     assert_eq!(e.to_string(), "No files match pattern `**/*.{png,jpg,gif}`".to_string());
/// }
/// # Ok(()) }
/// ```
pub fn glob(patterns: &str) -> Result<Vec<PathBuf>, Error> {
    use globwalk::GlobWalkerBuilder;

    let files: Vec<_> = GlobWalkerBuilder::from_patterns(".", &[patterns])
        .max_depth(1)
        .build()?
        .into_iter()
        .filter_map(StdResult::ok)
        .map(|dir_entry| dir_entry.path().to_owned())
        .collect();

    ensure!(
        files.get(0).is_some(),
        "No files match pattern `{}`",
        patterns
    );

    Ok(files)
}
