# Commit message generator with serde and reqwest

Let's start a new tool that outputs helpful commit messages.
We can use it in case we can't come up with one ourselves,
but it's Friday afternoon and we want to get hell out of the office.

## Create a Cargo project

Let's start a new project called "commit-msg-gen":
`cargo new --bin commit-msg-gen`.

You'll find a `Cargo.toml` file that contains:

```toml file=Cargo.toml
[package]
name = "commit-msg-gen"
version = "0.1.0"
authors = ["Your Name <your@email.address>"]

[dependencies]
```

As always,
add quicli (as well as structopt and serde) as dependencies:

```toml file=Cargo.toml
quicli = "0.1"
structopt = "0.1"
serde = "1"
```

We'll get out commit messages from the glorious website
called [whatthecommit.com],
so we'll need a crate that let's us do HTTP ws.
Let's use [reqwest]:

[whatthecommit.com]: https://whatthecommit.com/
[reqwest]: https://docs.rs/reqwest

```toml file=Cargo.toml
reqwest = "0.8"
```

## Import quicli and reqwest

To get this party started,
import our new friends:

```rust file=src/main.rs
extern crate reqwest;

#[macro_use] extern crate quicli;
use quicli::prelude::*;
```

## Write a CLI struct

So, what should our command line interface look like?
Above, we talked about generating a commit message.
How about we go one step further and offer to generate _multiple_ commit messages?

Sounds great?
Let's do this!

```rust file=src/main.rs
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

Alright, let's get down to business.
If we open `https://whatthecommit.com/index.json` in a browser,
we see it is structured like this:

```json
{
  "permalink": "http://whatthecommit.com/f71fdcde399d6e26db9d66c6166c9b99",
  "commit_message": "Is there an achievement for this?",
  "hash": "f71fdcde399d6e26db9d66c6166c9b99"
}
```

So we need to do two things:

1. Load that page and get its content somehow.
2. Parse the JSON data and get the commit message out of it.

For the first, we've already imported [reqwest],
so it's only a function call away (nice!):

```rust
reqwest::get("https://whatthecommit.com/index.json")?
```

To parse data in a format like JSON, there is serde.
Remeber that we added that as dependency earlier, too?
As it turns out, reqwest has a handy `.json()` method available
on its HTTP response type, that uses serde under the hood.
So, we can do:

```rust
reqwest::get("https://whatthecommit.com/index.json")?.json()?;
```

What does this return?
Anything you like!
Well, anything you like _and_ that serde can produce from JSON.
With a small type annotation you can tell the compiler what type you expect,
and serde will try to generate it from the JSON input.

If you've used dynamically typed languages like JavaScript before,
you might expect to get a general object that contains JSON-like data.
This is also possible in Rust:
The serde_json crate has a type called [`Value`],
that maps directly to what JSON looks like.
(So, a number, a string, a boolean, an array, a map, or nested versions of these.)
But there is another option,
and it is suprisingly easy to use but gives us a lot of benefits.
We can define our own type that describes the JSON structure:

[`Value`]: https://docs.rs/serde_json/1.0.9/serde_json/enum.Value.html

```rust file=src/main.rs
#[derive(Deserialize)]
struct Commit {
    commit_message: String,
}
```

The interesting thing here is the `#[derive(Deserialize)]` annotation.
It generates an implementation of a special trait so serde can parse data
and turn it into `Commit`s.
In the above definition, we only wrote one of the three fields.
That's okay, the other fields will be ignored.
We don't have to care about those right now.

Nice!
Now, with that out of the way,
let's write the typical `main!` macro,
but this time with a loop
in which we request some wonderful commits,
and print them.
Oh, and because it might take a while for the commits to arrive,
we also write some log output.
(Use `-vv` to see it!)

```rust file=src/main.rs
main!(|args: Cli, log_level: verbosity| {
    for i in 0..args.amount {
        info!("try {}", i);
        let c: Commit = reqwest::get("https://whatthecommit.com/index.json")?.json()?;
        println!("{}) {}", i + 1, c.commit_message);
    }
});
```

## Give it a spin!

All set? `cargo run` it!
