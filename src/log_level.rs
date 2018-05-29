#![allow(missing_docs)]

use log::{Level, LevelFilter};
use env_logger::Builder as LoggerBuilder;
use failure::Error;

/// Easily add a `--verbose` flag to CLIs using Structopt
///
/// # Examples
///
/// ```rust
/// extern crate quicli;
/// #[macro_use] extern crate structopt;
///
/// use structopt::StructOpt;
/// use quicli::prelude::Verbosity;
///
/// /// Le CLI
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///     #[structopt(flatten)]
///     verbose: Verbosity,
/// }
/// #
/// # fn main() {}
/// ```
#[derive(StructOpt, Debug)]
pub struct Verbosity {
    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors. Passing `-v` one time also prints
    /// warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

impl Verbosity {
    fn log_level(&self) -> LevelFilter {
        match self.verbosity {
            0 => Level::Error,
            1 => Level::Warn,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }.to_level_filter()
    }

    /// Initialize `env_logger` and set the log level for the given package.
    ///
    /// All other modules default to printing warnings.
    pub fn setup_env_logger(&self, own_pkg_name: &str) -> Result<(), Error> {
        LoggerBuilder::new()
            .filter(Some(&own_pkg_name.replace("-", "_")), self.log_level())
            .filter(None, Level::Warn.to_level_filter())
            .try_init()?;
        Ok(())
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity)
    }
}
