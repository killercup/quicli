//! Some helpful functions to deal with file system operations.
//!
//! These are rather simple, but provide a quick and easy way to to common
//! tasks. Also, they have great error messages.

use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;

use failure::{Error, ResultExt};

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
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();
    ensure!(path.exists() && path.is_file(), "Path {:?} is not a file!", path);

    let mut file = File::open(path)
        .with_context(|_| format!("Could not open file {:?}", path))?;
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
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
    let path = path.as_ref();

    let mut f = File::create(path)
        .with_context(|_| format!("Could not create/open file {:?}", path))?;

    f.write_all(content.as_bytes())
        .with_context(|_| format!("Could not write to file {:?}", path))?;

    Ok(())
}
