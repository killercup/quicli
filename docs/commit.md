# Generate a commit message

Let's start a new tool that outputs helpful commit messages.
We can use it in case we can't come up with one ourselves,
but it's Friday afternoon and we want to get hell out of the office.

## Create a Cargo project

Let's start a new project called "commit-msg-gen":
`cargo new --bin commit-msg-gen`.

You'll find a `Cargo.toml` file that contains:

```toml,file=Cargo.toml
[package]
name = "commit-msg-gen"
version = "0.1.0"
authors = ["Your Name <your@email.address>"]

[dependencies]
```

As always,
add quicli as well as structopt as dependencies:

```toml,file=Cargo.toml
quicli = "0.1"
structopt = "0.1"
serde = "1"
```

We'll get out commit messages from the glorious website
called [whatthecommit.com],
so we'll need a crate that let's us do HTTP requests.
Let's use [request]:

[whatthecommit.com]: https://whatthecommit.com/
[request]: https://docs.rs/reqwest

```toml,file=Cargo.toml
reqwest = "0.8"
```

## Import quicli and reqwest

To get this party started,
import our new friends:

```rust,file=src/main.rs
extern crate reqwest;

#[macro_use] extern crate quicli;
use quicli::prelude::*;
```

## Write a CLI struct

So, what should our command line interface look like?
Let's try this:

```rust,file=src/main.rs
/// Get some cool commit messages!
#[derive(Debug, StructOpt)]
struct Cli {
    /// How many?
    #[structopt(long = "amount", default_value = "3")]
    amount: i32,
    /// Pass many times for more log output
    #[structopt(long = "verbosity", short = "v")]
    verbosity: u64,
}
```

### Implement all the features

<!-- TODO: Serde -->
<!-- TODO: What the commit structure -->

```rust,file=src/main.rs
#[derive(Debug, Deserialize)]
struct Commit {
    commit_message: String,
}
```


```rust,file=src/main.rs
main!(|args: Cli, log_level: verbosity| {
    for i in 0..args.amount {
        info!("try {}", i);
        let c: Commit = reqwest::get("https://whatthecommit.com/index.json")?.json()?;
        println!("{}) {}", i + 1, c.commit_message);
    }
});
```

<!-- TODO: logging-->

## Give it a spin!

<!-- TODO -->
