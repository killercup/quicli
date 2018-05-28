//! Some tools to get you started with writing small CLIs in Rust.
//!
//! You can find some examples and more information on how to use this crate
//! in the [README](https://github.com/killercup/quicli).

#![allow(unused_imports)]
#![deny(missing_docs)]

#[cfg(feature = "full-throttle")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "full-throttle")]
extern crate serde;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(feature = "full-throttle")]
extern crate rayon;

#[cfg(feature = "full-throttle")]
pub mod fs;
mod main_macro;

mod reexports {
    #[cfg(feature = "full-throttle")]
    #[doc(hidden)]
    pub use serde_derive::*;

    #[doc(hidden)]
    pub use structopt_derive::*;
    #[doc(hidden)]
    pub mod structopt {
        pub use structopt::*;
    }
    #[doc(hidden)]
    pub use structopt::StructOpt;

    #[doc(hidden)]
    pub use failure_derive::*;
    #[doc(hidden)]
    pub use failure::*;

    #[doc(hidden)]
    pub use log::*;

    #[cfg(feature = "full-throttle")]
    pub use rayon::prelude::*;
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
    pub type Result<T> = ::std::result::Result<T, ::failure::Error>;

    #[cfg(feature = "full-throttle")]
    pub use fs::*;

    #[doc(hidden)]
    pub use env_logger::Builder as LoggerBuilder;
    #[doc(hidden)]
    pub use log::Level as LogLevel;
}
