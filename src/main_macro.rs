/// Quickly get a good `main` function
///
/// Inside the block, you can write code using `?`. An `Ok(())` will
/// automatically be appended. The macro will also set up logging (using
/// `env_logger`) automatically.
///
/// # Parameters
///
/// You can optionally call this macro with a closure-like syntax to get some
/// default bindings:
///
/// - `main!(|args: Cli| { ... })`: Automatically parses command line flags into
///   a struct `Cli` and binds it to `args`. (Basically, it prepends
///   `let args = Cli::from_args();`).
/// - `main!(|args: Cli, log_level: verbosity| { ... })`: The log level will
///   depend on the integer value of the `verbosity` field of `args` (error = 0,
///   warn = 1, info = 2, debug = 3, trace = ≥4). The field's type should be
///   set to `parse(from_occurrences)` in the `#[structopt(…)]` attribute,
///   so structopt will automatically give you its number of occurances..
///
/// # Examples
///
/// ```rust,ignore
/// #[macro_use] extern crate quicli;
/// use quicli::prelude::*;
///
/// main!({
///     let x = read_file(".gitignore")?;
///     println!("{}", x);
/// });
/// ```
#[macro_export]
macro_rules! main {
    (|$args:ident: $cli:ty, log_level: $verbosity:ident| $body:expr) => {
        fn main() {
            fn run() -> $crate::prelude::Result<()> {
                let $args = <$cli>::from_args();
                $args.$verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

                $body

                Ok(())
            }

            if let Err(e) = run() {
                eprintln!("error: {}", e);
                for cause in e.iter_causes() {
                    eprintln!("caused by: {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };

    (|$args:ident: $cli:ty| $body:expr) => {
        fn main() {
            fn run() -> $crate::prelude::Result<()> {
                let $args = <$cli>::from_args();
                $crate::prelude::LoggerBuilder::new()
                    .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")), $crate::prelude::LogLevel::Error.to_level_filter())
                    .filter(None, $crate::prelude::LogLevel::Warn.to_level_filter())
                    .try_init()?;

                $body

                Ok(())
            }

            if let Err(e) = run() {
                eprintln!("error: {}", e);
                for cause in e.causes().skip(1) {
                    eprintln!("caused by: {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };

    (|| $body:expr) => { main!($body); };

    ($body:expr) => {
        fn main() {
            fn run() -> $crate::prelude::Result<()> {
                $crate::prelude::LoggerBuilder::new()
                    .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")), $crate::prelude::LogLevel::Error.to_level_filter())
                    .filter(None, $crate::prelude::LogLevel::Warn.to_level_filter())
                    .try_init()?;

                $body
                Ok(())
            }

            if let Err(e) = run() {
                eprintln!("error: {}", e);
                for cause in e.causes().skip(1) {
                    eprintln!("caused by: {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };
}
