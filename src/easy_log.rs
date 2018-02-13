use super::prelude;

/// Set the logs verbosity based on an integer value:
///
/// - `0`: error
/// - `1`: warn
/// - `2`: info
/// - `3`: debug
/// - `>=4`: trace
///
/// This is used in the [`main!`] macro. This function is _not_ stabilized and should
/// not (yet) be used directly. See
///
/// [`main!`]: macro.main.html
#[doc(hidden)]
pub fn set_log_verbosity(verbosity: u64) -> prelude::Result<()> {
    let log_level = match verbosity {
        0 => prelude::LogLevel::Error,
        1 => prelude::LogLevel::Warn,
        2 => prelude::LogLevel::Info,
        3 => prelude::LogLevel::Debug,
        _ => prelude::LogLevel::Trace,
    }.to_level_filter();

    prelude::LoggerBuiler::new()
        .filter(Some(env!("CARGO_PKG_NAME")), log_level)
        .filter(None, prelude::LogLevel::Warn.to_level_filter())
        .try_init()?;
    Ok(())
}
