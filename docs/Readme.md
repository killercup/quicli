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

[dependencies]
```

Add _quicli_ as an dependency by adding this line
to the `Cargo.toml` file:

```toml file=Cargo.toml
quicli = "0.1"
```

To be able to use _all_ the features,
also add these two goodies
just below the "quicli" line:

```toml file=Cargo.toml
structopt = "0.1"
serde = "1"
```

## Import quicli

Now, it's time to get started with writing some Rust!
Open up your `src/main.rs`.
Let's import all the good stuff:

```rust file=src/main.rs
#[macro_use] extern crate quicli;
use quicli::prelude::*;
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
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v")]
    verbosity: u64,
}
```

You can find out about the possible attributes on the Cli struct in
[structopt's documentation].

[structopt's documentation]: https://docs.rs/structopt-derive/0.1.6/structopt_derive/

## Implement all the features

The next step is the easiest one yet;
You just have to implement all the features you want to add!
For now, let's leave it at this:

```rust file=src/main.rs
main!(|args: Cli, log_level: verbosity| {
    let data = read_file(&args.file)?;
    info!("Reading first {} lines of {:?}", args.count, args.file);
    data.lines().take(args.count).for_each(|line| println!("{}", line));
});
```

You can find out more about the main macro in [quicli's API documentaton].

[quicli's API documentaton]: https://docs.rs/quicli/0.1.1/quicli/macro.main.html

## Give it a spin!

1. `cargo run` it! Did you see a nice error?
2. What does `cargo run -- Cargo.toml` show you?
3. How about `cargo run -- Cargo.toml --count=4` or `cargo run -- Cargo.toml -n 2`?
4. `cargo run -- --help` -- how cool is that?
5. More fun: Try `--cont 4` (with the missing u).
6. Do you like log messages? That's what we added the `verbosity` field for!
    Give `cargo run -- Cargo.toml -vv` a try!
