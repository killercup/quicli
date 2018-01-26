//! Generate jpg thumbnails in parallel
//!
//! From <https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html#generate-jpg-thumbnails-in-parallel>

extern crate image;
#[macro_use]
extern crate quicli;
extern crate structopt;

use quicli::prelude::*;

use std::path::{Path, PathBuf};
use image::FilterType;

/// Make some thumbnails
#[derive(Debug, StructOpt)]
struct Cli {
    /// Which files?
    #[structopt(default_value = "*.jpg")]
    pattern: String,
    /// How long should the longest edge of the thumbnail be?
    #[structopt(long = "max-size", short = "s", default_value = "300")]
    size: u32,
    /// Where do you want to save the thumbnails?
    #[structopt(long = "output", short = "o", default_value = "thumbnails")]
    thumb_dir: String,
    /// Pass many times for more log output
    #[structopt(long = "verbosity", short = "v")]
    verbosity: u64,
}

main!(|args: Cli, log_level: verbosity| {
    let files: Vec<_> = glob(&args.pattern)?.filter_map(|x| x.ok()).collect();
    create_dir(&args.thumb_dir)?;

    info!("Saving {} thumbnails into {:?}...", files.len(), args.thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, &args.thumb_dir, args.size).map_err(|e| {
                // Print error immediately
                error!("failed to resize {:?} ({})", path, e);
                ()
            })
        })
        .filter_map(|x| x.err())
        .collect();

    println!(
        "{} of {} thumbnails saved successfully",
        files.len() - image_failures.len(),
        files.len()
    );
});

/// Resize `original` to have a maximum dimension of `longest_edge` and save the
/// resized image to the `thumb_dir` folder
fn make_thumbnail(original: &Path, thumb_dir: &str, longest_edge: u32) -> Result<()> {
    let thumb_path = PathBuf::from(thumb_dir).join(original);

    let img = image::open(&original)?;
    let fout = &mut std::fs::File::create(thumb_path)?;

    img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(fout, image::JPEG)?;

    Ok(())
}
