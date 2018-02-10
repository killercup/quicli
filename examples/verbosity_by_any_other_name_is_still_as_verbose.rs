#[macro_use]
extern crate quicli;
use quicli::prelude::*;

/// Verbosity by any other name is still as verbose
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    team_rockets_blasting_off_again: u64
}

main!(|cli_args: Cli, log_level: team_rockets_blasting_off_again| {
    println!("{} verbose flags passed", cli_args.team_rockets_blasting_off_again);

    error!("always shown");
    if log_enabled!(Level::Error) {
        println!("error logging enabled");
    } else {
        println!("error debugging disabled");
    }

    warn!("shown at -v");
    if log_enabled!(Level::Warn) {
        println!("warn logging enabled");
    } else {
        println!("warn debugging disabled");
    }

    info!("shown at -vv");
    if log_enabled!(Level::Info) {
        println!("info logging enabled");
    } else {
        println!("info debugging disabled");
    }

    debug!("shown at -vvv");
    if log_enabled!(Level::Debug) {
        println!("debug logging enabled");
    } else {
        println!("debug debugging disabled");
    }

    trace!("shown at -vvvv");
    if log_enabled!(Level::Trace) {
        println!("trace debugging enabled");
    } else {
        println!("trace debugging disabled");
    }
});
