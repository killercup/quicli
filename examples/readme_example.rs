#[macro_use] extern crate quicli;
use quicli::prelude::*;

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
