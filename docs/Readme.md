# Get started with quicli

quicli is a neat little framework for quickly writing CLI applications in Rust.

We carefully selected the best tools the ecosystem has to offer,
coupled them with an opinioned setup,
and wrote some helpful guides.
quicli enables you to concentrate on your application code,
and gives you features like
powerful CLI argument handling,
great error messages,
and logging
without you needing to think about it.

But enough of the sales talk, let's build something!
How about this:
We are going to create a small CLI tool
that outputs the first `n` lines of a given file.

_Note:_
While we hope quicli is great tool to get started with Rust
this documentation is not an introduction to the language.
We recommend you to read [the book] first.

[the book]: https://doc.rust-lang.org/book/

## Create a Cargo project

Create a new Rust binary project called "head"
with `cargo new --bin head`.
You should end up with a `Cargo.toml` that looks like this:

```toml file=Cargo.toml
[package]
name = "head"
version = "0.1.0"
authors = ["Your Name <your@email.address>"]
edition = "2018"

[dependencies]
```

Add _quicli_ as an dependency,
as well as _structopt_
(for dealing with command line arguments)
by adding this to the `Cargo.toml` file:

```toml file=Cargo.toml
quicli = "0.3"
structopt = "0.2"
```

## Import quicli

Now, it's time to get started with writing some Rust!
Open up your `src/main.rs`.
Let's import all the good stuff:

```rust file=src/main.rs
use quicli::prelude::*;
use structopt::StructOpt;
```

That's it. That's all the imports you should need for now!

## Write a CLI struct

Now, quickly write a cool CLI
(it's also okay to type slowly):

```rust file=src/main.rs
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
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}
```

You can find out about the possible attributes on the Cli struct in
[structopt's documentation].

[structopt's documentation]: https://docs.rs/structopt/0.2.0/structopt/

## Implement all the features

The next step is the easiest one yet;
You just have to implement all the features you want to add!
Just kidding, let's take it one step at a time.

We'll start with a function called `main`.
This is where our program starts,
calls other functions,
and then ends.

You might have seen that you can use
[the `?` operator][try-op]
in Rust function to propagate errors.
This is super convenient to deal with errors.
To be able to use it,
our main function needs to return a `Result`.
For that,
quicli contains a handy type called `CliResult`,
that will return pretty errors.
At the end of `main`,
we then return `Ok(())`,
(a value that says "everything okay, nothing else").

[try-op]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors

Next, we'll want to enable logging.
For that we'll use the `verbosity` field we added earlier.
It is of a special type that you can call `setup_env_logger` on,
with the name of the crate you want to log by default.

Alright, are you all set?
Then let's implement `head`!

```rust file=src/main.rs
fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;

    let content = read_file(&args.file)?;
    let content_lines = content.lines();
    let first_n_lines = content_lines.take(args.count);

    info!("Reading first {} lines of {:?}", args.count, args.file);

    for line in first_n_lines {
        println!("{}", line);
    }

    Ok(())
}
```

Alternatively, you could also write this more concisely
(by chaining the [Iterator] methods):

[Iterator]: https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html

```rust
fn main() -> CliResult {
    let args = Cli::from_args();

    read_file(&args.file)?
        .lines()
        .take(args.count)
        .for_each(|line| println!("{}", line));
};
```

## Give it a spin!

1. `cargo run` it! Did you see a nice error?
2. What does `cargo run -- Cargo.toml` show you?
3. How about `cargo run -- Cargo.toml --count=4` or `cargo run -- Cargo.toml -n 2`?
4. `cargo run -- --help` -- how cool is that?
5. More fun: Try `--cont 4` (with the missing u).
6. Do you like log messages? That's what we added the `verbosity` field for!
    Give `cargo run -- Cargo.toml -vv` a try!
