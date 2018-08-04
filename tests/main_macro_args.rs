#[macro_use] extern crate quicli;
use quicli::prelude::*;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The config file for the cli app.
    #[structopt(long = "config", short = "c", parse(from_os_str))]
    config: PathBuf,
}

main!(|args: Cli| {
    let x = read_file(&args.config)?;
    println!("{}", x);
});
