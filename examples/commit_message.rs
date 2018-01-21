extern crate reqwest;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

/// Get some cool commit messages!
#[derive(Debug, StructOpt)]
struct Cli {
    /// How many?
    #[structopt(long = "amount", default_value = "3")]
    amount: i32,
}

#[derive(Debug, Deserialize)]
struct Commit {
    commit_message: String,
}

main!({
    let args = Cli::from_args();

    for i in 0..args.amount {
        let c: Commit = reqwest::get("https://whatthecommit.com/index.json")?.json()?;
        println!("{}) {}", i + 1, c.commit_message);
    }
});
