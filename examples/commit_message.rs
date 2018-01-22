extern crate reqwest;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

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

#[derive(Debug, Deserialize)]
struct Commit {
    commit_message: String,
}

main!(|args: Cli, log_level: verbosity| {
    for i in 0..args.amount {
        info!("try {}", i);
        let c: Commit = reqwest::get("https://whatthecommit.com/index.json")?.json()?;
        println!("{}) {}", i + 1, c.commit_message);
    }
});
