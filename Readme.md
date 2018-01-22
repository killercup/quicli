# QuiCLI

Quickly build cool CLI apps in Rust.

[![Build Status](https://travis-ci.org/killercup/quicli.svg)][Travis]
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/quicli.svg)
[![crates.io](https://img.shields.io/crates/v/quicli.svg)][Crates.io]

[Travis]: https://travis-ci.org/killercup/quicli
[Crates.io]: https://crates.io/crates/quicli
[Documentation]: https://docs.rs/quicli

## Get started

1. Create a new Rust binary project called "head" with `cargo new --bin head`.
2. Add _quicli_ as an dependency in your `Cargo.toml`:

    ```toml
    [dependencies]
    quicli = "0.1"
    ```
    
    And, to be able to use all the features, also add these two goodies:
    
    ```toml
    structopt = "0.1"
    serde = "1"
    ```

3. Now, open up your `src/main.rs`. First, let's import all the good stuff:

    ```rust
    #[macro_use] extern crate quicli;
    use quicli::prelude::*;
    ```

4. Now, quickly write a cool CLI (it's also okay to type slowly):

    ```rust
    // Add cool slogan for your app here, e.g.:
    /// Get first n lines of a file
    #[derive(Debug, StructOpt)]
    struct Cli {
        // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
        /// How many lines to get
        #[structopt(long = "count", short = "n", default_value = "3")]
        count: usize,
        // Add a positional argument that the user has to supply:
        /// The file to read
        file: String,
    }

    main!({
        let args = Cli::from_args();
        let data = read_file(args.file)?;
        data.lines().take(args.count).for_each(|line| println!("{}", line));
    });
    ```

5. Give it a spin!

    1. `cargo run` it! Did you see a nice error?
    2. What does `cargo run -- Cargo.toml` show you?
    3. How about `cargo run -- Cargo.toml --count=4` or `cargo run -- Cargo.toml -n 2`?
    4. `cargo run -- --help` -- how cool is that?
    5. More fun: Try `--cont 4` (with the missing u).

## Thanks

This is only possible because of all the awesome libraries included here:

- Structopt and Clap for the nice CLI with awesome argument handling, great
  error messages, and nice composability!
- Serde for handling all things serializing and deserializing
- failure for ergonomic error handling.
- …and more to come!

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
