/// Quickly get a good `main` function
///
/// Inside the block, you can write code using `?`. An `Ok(())` will
/// automatically be appended.
///
/// _Note:_ This will be deprecated once the `try_main` feature is stabilized in
/// Rust.
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
    ($body:expr) => {
        fn main() {
            fn run() -> $crate::prelude::Result<()> {
                $body
                Ok(())
            }

            match run() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    ::std::process::exit(1);
                }
            }
        }
    };
}
