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
Just kidding, let's take it one step at a time.

quicli comes with a handy macro called `main!`.
You can use it as an entry point in your program,
and instead of the usual `fn main`.
Its purpose it to reduce the amount boilerplate code you need write.
Currently, it gives you
access to the parsed CLI args,
sets up logging,
and let's you use [the `?` operator][try-op].

[try-op]: https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html#propagating-errors

The content of `main!` looks like a closure,
and you can specify up to two parameters (they are both optional):

1. The CLI arguments
   (i.e., if you write `args: Cli` you get an `args` of the `Cli` type defined above)
2. The field in your `Cli` struct that defines the log level
   (just specify the field name and the macro will set up logging automatically)

In the body of this "closure" you can write regular Rust code,
just like in a `fn main`.
The only noticable difference is that you can use `?`
to exit the function on errors
and print a nice human-readable error message.
You can find out more about the main macro in [quicli's API documentation].

[quicli's API documentaton]: https://docs.rs/quicli/0.1.2/quicli/macro.main.html

Alright, are you all set?
Then let's implement `head`!

```rust file=src/main.rs
main!(|args: Cli, log_level: verbosity| {
    let content = read_file(&args.file)?;
    let content_lines = content.lines();
    let first_n_lines = content_lines.take(args.count);
    
    info!("Reading first {} lines of {:?}", args.count, args.file);

    for line in first_n_lines {
        println!("{}", line);
    }
});
```

Alternatively, you could also write this more concisely
(by chaining the [Iterator] methods):

[Iterator]: https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html

```rust
main!(|args: Cli, log_level: verbosity| {
    read_file(&args.file)?
        .lines()
        .take(args.count)
        .for_each(|line| println!("{}", line));
});
```

## Give it a spin!

1. `cargo run` it! Did you see a nice error?
2. What does `cargo run -- Cargo.toml` show you?
3. How about `cargo run -- Cargo.toml --count=4` or `cargo run -- Cargo.toml -n 2`?
4. `cargo run -- --help` -- how cool is that?
5. More fun: Try `--cont 4` (with the missing u).
6. Do you like log messages? That's what we added the `verbosity` field for!
    Give `cargo run -- Cargo.toml -vv` a try!
