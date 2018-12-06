use quicli::prelude::*;
use structopt::StructOpt;

/// Verbosity by any other name is still as verbose
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    team_rockets_blasting_off_again: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.team_rockets_blasting_off_again.setup_env_logger(&env!("CARGO_PKG_NAME"))?;
    println!("{} verbose flags passed", args.team_rockets_blasting_off_again);

    error!("always shown");
    if log_enabled!(log::Level::Error) {
        println!("error logging enabled");
    } else {
        println!("error debugging disabled");
    }

    warn!("shown at -v");
    if log_enabled!(log::Level::Warn) {
        println!("warn logging enabled");
    } else {
        println!("warn debugging disabled");
    }

    info!("shown at -vv");
    if log_enabled!(log::Level::Info) {
        println!("info logging enabled");
    } else {
        println!("info debugging disabled");
    }

    debug!("shown at -vvv");
    if log_enabled!(log::Level::Debug) {
        println!("debug logging enabled");
    } else {
        println!("debug debugging disabled");
    }

    trace!("shown at -vvvv");
    if log_enabled!(log::Level::Trace) {
        println!("trace debugging enabled");
    } else {
        println!("trace debugging disabled");
    }

    Ok(())
}
