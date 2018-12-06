use quicli::prelude::*;
use structopt::StructOpt;
/// Get some cool commit messages!
#[derive(Debug, StructOpt)]
struct Cli {
    /// How many?
    #[structopt(long = "amount", default_value = "3")]
    amount: i32,
    #[structopt(flatten)]
    verbosity: Verbosity,
}
#[derive(Deserialize)]
struct Commit {
    commit_message: String,
}
fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("commit-msg-gen")?;

    for i in 0..args.amount {
        info!("try {}", i);
        let c: Commit = reqwest::get("https://whatthecommit.com/index.json")?.json()?;
        println!("{}) {}", i + 1, c.commit_message);
    }

    Ok(())
}
