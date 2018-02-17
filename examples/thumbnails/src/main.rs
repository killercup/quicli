#[macro_use] extern crate quicli;
extern crate image;
use quicli::prelude::*;
/// Make some thumbnails
#[derive(Debug, StructOpt)]
struct Cli {
    /// Pass many times for more log output
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Which files?
    #[structopt(default_value = "*.jpg")]
    pattern: String,
    /// How long should the longest edge of the thumbnail be?
    #[structopt(long = "max-size", short = "s", default_value = "300")]
    size: u32,
    /// Where do you want to save the thumbnails?
    #[structopt(long = "output", short = "o", default_value = "thumbnails")]
    thumb_dir: String,
}
main!(|args: Cli, log_level: verbosity| {
    let files = glob(&args.pattern)?;
    create_dir(&args.thumb_dir)?;
    info!("Saving {} thumbnails into {:?}...", files.len(), args.thumb_dir);
use std::path::Path;

fn make_thumbnail(
    original: &Path,
    thumb_dir: &str,
    longest_edge: u32,
) -> Result<()> {
    let img = image::open(&original)?;
    let thumbnail = img.resize(longest_edge, longest_edge, image::FilterType::Nearest);
    use std::path::PathBuf;
    use std::fs::File;

    let thumb_name = original
        .file_name()
        .ok_or_else(|| format_err!("couldn't read file name of {:?}", original))?;
    let thumb_path = PathBuf::from(thumb_dir)
        .join(thumb_name)
        .with_extension("jpg");

    let mut output_file = File::create(thumb_path)?;
    thumbnail.save(&mut output_file, image::JPEG)?;
    Ok(())
}
    let thumbnails = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, &args.thumb_dir, args.size)
            .map_err(|e| error!("failed to resize {} ({})", path.display(), e))
        });
    let thumbnail_count: i32 = thumbnails
        .map(|x| if x.is_ok() { 1 } else { 0 })
        .sum();
    println!(
        "{} of {} files successfully thumbyfied!",
        thumbnail_count,
        files.len()
    );
});
