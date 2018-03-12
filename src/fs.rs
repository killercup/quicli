//! Some helpful functions to deal with file system operations.
//!
//! These are rather simple, but provide a quick and easy way to to common
//! tasks. Also, they have great error messages.

extern crate glob;

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Read, Write};
use std::fs::File;
use std::result::Result as StdResult;

use failure::{Error, ResultExt};

use prelude::Result;

pub use std::fs::create_dir_all as create_dir;

/// Read file content into string
///
/// # Examples
///
/// ```rust
/// # extern crate quicli;
/// # use quicli::prelude::*;
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<()> {
/// let x = read_file(".gitignore")?;
/// assert!(x.len() > 0);
/// # Ok(()) }
/// ```
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
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
/// # extern crate quicli;
/// # use quicli::prelude::*;
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<()> {
/// write_to_file("/tmp/asdasidz81zasda", "foobar")?;
/// # Ok(()) }
/// ```
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
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
/// # extern crate quicli;
/// # use quicli::prelude::*;
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<()> {
/// let markdown_files = glob("*.md")?;
/// assert_eq!(markdown_files.len(), 2);
///
/// let weird_files = glob("**/*.weird");
/// assert!(weird_files.is_err());
/// if let Err(e) = weird_files {
///     assert_eq!(e.to_string(), "No files match pattern `**/*.weird`".to_string());
/// }
/// # Ok(()) }
/// ```
pub fn glob(pattern: &str) -> Result<Vec<PathBuf>> {
    use self::glob::{glob_with, MatchOptions};

    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with(pattern, &options)?
        .filter_map(StdResult::ok)
        .collect();

    ensure!(
        files.get(0).is_some(),
        "No files match pattern `{}`",
        pattern
    );

    Ok(files)
}
