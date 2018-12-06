//! Some tools to get you started with writing small CLIs in Rust.
//!
//! You can find some examples and more information on how to use this crate
//! in the [README](https://github.com/killercup/quicli).

#![deny(missing_docs)]

#[cfg(feature = "full-throttle")]
pub mod fs;
mod main_macro;

mod reexports {
    #[cfg(feature = "full-throttle")]
    #[doc(hidden)]
    pub use serde_derive::{Deserialize, Serialize};

    #[doc(hidden)]
    pub use structopt_derive::*;
    #[doc(hidden)]
    pub mod structopt {
        pub use structopt::*;
    }
    #[doc(hidden)]
    pub use structopt::StructOpt;

    pub use clap_verbosity_flag::Verbosity;

    #[doc(hidden)]
    pub use failure_derive::Fail;
    #[doc(hidden)]
    pub use failure::{Error, ResultExt, bail, ensure, err_msg};

    #[doc(hidden)]
    pub use log::{error, warn, info, debug, trace};

    #[cfg(feature = "full-throttle")]
    pub use rayon::prelude::*;

    #[doc(hidden)]
    pub use env_logger::Builder as LoggerBuilder;
    #[doc(hidden)]
    pub use log::Level as LogLevel;
}

/// Prelude – import all of this
///
/// To get up-and-running _real_ fast, do `use quicli::prelude::*`. It's just
/// like `use std::io::prelude::*` but with less I/O and more C/L/I!
///
/// (If you don't like glob imports, feel free to import the items 1-by-1. You
/// will sadly miss out on a bunch of derive macros, though.)
pub mod prelude {
    pub use reexports::*;

    /// A handy alias for `Result` that carries a generic error type.
    pub type CliResult<T> = ::std::result::Result<T, ::exitfailure::ExitFailure>;

    #[cfg(feature = "full-throttle")]
    pub use fs::*;
}
