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
/// Most basic form.
///
/// ```rust,no_run
/// #[macro_use] extern crate quicli;
/// use quicli::prelude::*;
///
/// main!({
///     let x = read_file(".gitignore")?;
///     println!("{}", x);
/// });
/// ```
///
/// With command line arguments.
///
/// ```rust,ignore
/// #[macro_use] extern crate quicli;
/// use quicli::prelude::*;
/// use std::path::PathBuf;
///
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///     /// The config file for the cli app.
///     #[structopt(long = "config", short = "c", parse(from_os_str))]
///     config: PathBuf,
/// }
///
/// main!(|args: Cli| {
///     let x = read_file(&args.config)?;
///     println!("{}", x);
/// });
/// ```
///
/// With command line arguments including a verbosity flag.
///
/// ```rust,ignore
/// #[macro_use] extern crate quicli;
/// use quicli::prelude::*;
/// use std::path::PathBuf;
///
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///     /// The config file for the cli app.
///     #[structopt(long = "config", short = "c", parse(from_os_str))]
///     config: PathBuf,
///     #[structopt(flatten)]
///     verbosity: Verbosity,
/// }
///
/// main!(|args: Cli, log_level: Verbosity| {
///     let x = read_file(&args.config)?;
///     println!("{}", x);
/// });
/// ```
#[macro_export]
macro_rules! main {
    (|$args:ident: $cli:ty, log_level: $verbosity:ident| $body:expr) => {
        fn main() {
            fn run($args: $cli) -> $crate::prelude::Result<()> {
                $body
                #[allow(unreachable_code)]
                Ok(())
            }

            let $args = <$cli>::from_args();
            // This cannot fail because we control the main method, so no-one else can register a
            // log handler before us.
            $args.$verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))
                .expect("logger setup should never fail, this is a bug in quicli");
            if let Err(e) = run($args) {
                error!("{}", e);
                for cause in e.iter_causes() {
                    error!("caused by {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };

    (|$args:ident: $cli:ty| $body:expr) => {
        fn main() {
            fn run($args: $cli) -> $crate::prelude::Result<()> {
                $body
                #[allow(unreachable_code)]
                Ok(())
            }

            let $args = <$cli>::from_args();
            $crate::prelude::LoggerBuilder::new()
                .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")),
                        $crate::prelude::LogLevel::Error.to_level_filter())
                .filter(None, $crate::prelude::LogLevel::Warn.to_level_filter())
                .try_init().expect("logger setup should never fail, this is a bug in quicli");

            if let Err(e) = run($args) {
                error!("{}", e);
                for cause in e.iter_causes() {
                    error!("caused by {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };

    (|| $body:expr) => { main!($body); };

    ($body:expr) => {
        fn main() {
            fn run() -> $crate::prelude::Result<()> {
                $body
                #[allow(unreachable_code)]
                Ok(())
            }

            $crate::prelude::LoggerBuilder::new()
                .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")),
                        $crate::prelude::LogLevel::Error.to_level_filter())
                .filter(None, $crate::prelude::LogLevel::Warn.to_level_filter())
                .try_init().expect("logger setup should never fail, this is a bug in quicli");

            if let Err(e) = run() {
                error!("{}", e);
                for cause in e.iter_causes() {
                    eprintln!("caused by {}", cause);
                }
                ::std::process::exit(1);
            }
        }
    };
}
