#[macro_use]
extern crate quicli;
use quicli::prelude::*;

/// Verbosity by any other name is still as verbose
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "verbose", short = "v")]
    team_rockets_blasting_off_again: u64
}

main!(|cli_args: Cli, log_level: team_rockets_blasting_off_again| {
    error!("always shown");
    warn!("shown at -v");
    info!("shown at -vv");
    debug!("shown at -vvv");
    trace!("shown at -vvvv");
});
