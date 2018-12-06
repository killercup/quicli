# Create thumbnails from JPG files

Let's write a CLI tool that renders small "thumbnail" version
of images matching a pattern.
And to make it more interesting than a boring old shell script,
let's do in with a bit of concurrency
and a nice user experience!

This is an adaptation from [this example][cookbook-thumb]
from the [Rust Cookbook].

[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook/
[cookbook-thumb]: https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html#generate-jpg-thumbnails-in-parallel

## Create a Cargo project

Let's start a new project called "thumbify":
`cargo new --bin thumbify`.

You'll find a `Cargo.toml` file that contains:

```toml file=Cargo.toml
[package]
name = "thumbify"
version = "0.1.0"
authors = ["Your Name <your@email.address>"]

[dependencies]
```

As always, add _quicli_ to your dependencies:

```toml file=Cargo.toml
quicli = "0.3"
```

Since we need to resize images,
we'll also need to import a library that can do that.
Let's use the one called "[image]."
It sounds like something that you can do picture-related things with.

[image]: https://docs.rs/image/0.18.0/image/

```toml file=Cargo.toml
image = "0.18"
```

## Import quicli and image

Let's ~~load~~ invite our new friends:

```rust file=src/main.rs
#[macro_use] extern crate quicli;
extern crate image;
```

And let's also grab everything from _quicli_:

```rust file=src/main.rs
use quicli::prelude::*;
```

What other stuff do we need?
Maybe something from the image crate?
Don't worry:
You don't need to know that upfront,
and it's fine to add more later on.

## Write a CLI struct

Alright!
Here we go:

```rust file=src/main.rs
/// Make some thumbnails
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,
```

So far so typical for a _quicli_ app.
Just the same as the example from the Getting Started guide.

Now, let's do a bit of thinking, though:
What do our users need to specify so we can thumbify their images?
Maybe a path to their directory that contains their images?
Or maybe a _pattern_ that matches only some of their files?

That 'pattern' idea does sounds more powerful, let's go with that.
But it also sounds more complicated,
so we'll see how difficult it will be to implement.
In any case, let's also provide a default value.

```rust file=src/main.rs
    /// Which files?
    #[structopt(default_value = "*.jpg")]
    pattern: String,
```

Next up: How large should these thumbnails be?
A non-negative integer seems like a good choice.

```rust file=src/main.rs
    /// How long should the longest edge of the thumbnail be?
    #[structopt(long = "max-size", short = "s", default_value = "300")]
    size: u32,
```

Anything else?
Ah, yes, actually:
Let's also add an option to specify _where_ to save those thumbnails!

```rust file=src/main.rs
    /// Where do you want to save the thumbnails?
    #[structopt(long = "output", short = "o", default_value = "thumbnails")]
    thumb_dir: String,
```

And a flag to specify wether we want to clean it before creating our thumbnails!

``` rust file=src/main.rs
    /// Should we clean the output directory?
    #[structopt(long="clean-dir")]
    clean_dir: bool,
```

There we go.
Oh, wait, let's not forget to close that struct definition:

```rust file=src/main.rs
}
```

Yeah, now we're done.

### Implement all the features

Onto implementing features!

```rust file=src/main.rs
main!(|args: Cli, log_level: verbosity| {
```

### Globs

First, the good news:
We don't need to care about the file path/name pattern matching stuff.
_quicli_ contains a `glob` function,
that, given something like `*.jpg`, `images/*.jpg`, or even `foo/**/bar*.gif`,
gives you a list of all the file paths that match the pattern.

```rust file=src/main.rs
    let files = glob(&args.pattern)?;
```

Before creating any of the thumbnails, let's clean-up the output directory,
if requested by the caller.
_quicli_ provides `remove_dir_all` to clean any directory tree you'd like!

```rust file=src/main.rs
    let thumb_dir = std::path::Path::new(&args.thumb_dir);
    if args.clean_dir && thumb_dir.exists() {
        remove_dir_all(&thumb_dir)?;
    }
```

Now we're ready to (re)create the output directory.
(another function _quicli_ gives you):

```rust file=src/main.rs
    create_dir(&thumb_dir)?;
```

Great, that was the first step.
If you're proud of that,
this is your chance to yell it from the mountain tops!

```rust file=src/main.rs
    info!("Saving {} thumbnails into {:?}...", files.len(), args.thumb_dir);
```

(Assumes people in the village are using `-vv` to get 'info' level logs.)

### Image resizing

Okay, on to the actual image file processing bit.
This is where we use the image crate to scale the files.

Let's write a function that takes our image path,
the settings we got from the CLI arguments,
and returns a Result
that is either `Ok` but contains no data
(it saves the new image file directly and doesn't return its data to us),
or is `Err` and carries some information about what went wrong.
_quicli_ contains a type alias for the usual `Result`,
that automatically sets the `Err` variant to _failure_'s [`Error`] type.
This will save you some typing in the common case.

[`Error`]: https://docs.rs/failure/0.1.1/failure/struct.Error.html

```rust file=src/main.rs
use std::path::Path;

fn make_thumbnail(
    original: &Path,
    thumb_dir: &str,
    longest_edge: u32,
) -> Result<(), Error> {
```

What a pretty function signature!
You try to make the signatures of these helper functions as readable as possible,
as they can often serve as documentation.
You are of course still free to add a documentation comment
with more information about what you intend them to be used for!

And while image's API documentation is a bit lacking right now,
the usage of the _image_ crate is quite simple.
We open the image file,
and call [`resize`] on it:

[`resize`]: https://docs.rs/image/0.18.0/image/imageops/fn.resize.html

```rust file=src/main.rs
    let img = image::open(&original)?;
    let thumbnail = img.resize(longest_edge, longest_edge, image::FilterType::Nearest);
```

Now, let's create the JPG file in our thumbnails directory:

```rust file=src/main.rs
    use std::path::PathBuf;
    use std::fs::File;

    let thumb_name = original
        .file_name()
        .ok_or_else(|| format_err!("couldn't read file name of {:?}", original))?;
    let thumb_path = PathBuf::from(thumb_dir)
        .join(thumb_name)
        .with_extension("jpg");

    let mut output_file = File::create(thumb_path)?;
```

Right now, this creates a thumbnail with the same name as the original file.
This is a good point to add some features to make this more clever/customizable!

And finally, save our thumbnail

```rust file=src/main.rs
    thumbnail.save(&mut output_file, image::JPEG)?;
```

Great, our job here is done!
If all went well, we just saved thumbnail file!
(Otherwise, the `?` will exit the function and return the error.)
We don't even need to return any data here, so let's just say everything is fine:

```rust file=src/main.rs
    Ok(())
}
```

### Concurrency

It's safe to assume that
we are doing some iterating over this `files` list.
Something like this probably:

```rust
files
    .iter()
    .map(|f| make_thumbnail(f, &args.thumb_dir, args.size))
```

But wait:
Do you remember how we said we wanted to render these thumbnails _in parallel_?
Good for us that one of Rust's slogans is "Fearless Concurrency", then!

All we need to do is change the above `.iter()` to `.par_iter()`
and thanks to the power of [Rayon] (which quilci includes)
we are good to go!

[Rayon]: https://docs.rs/rayon/

```rust file=src/main.rs
    let thumbnails = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, &args.thumb_dir, args.size)
```

Ah, wait, before we get ahead of ourselves,
let's talk a bit about what we want to do here.
What if `make_thumbnail` fails?
Do we just want to ignore the errors?
Quit the whole program?
A good middle-ground might be logging the errors.
This way, the user still sees that the program wasn't really successful,
but also gets as many thumbnails as possible.

Luckily, this is also the place where we have the most information:
We know all the arguments that we gave `make_thumbnail`
as well as its error message.
Let's use `.map_err` to capture the error and log something:

```rust file=src/main.rs
            .map_err(|e| error!("failed to resize {} ({})", path.display(), e))
        });
```

Good.
Now, the user will see some errors as they occur.
Maybe even a few at the same time!

Let's also do a "final" message when we are finished,
showing how many files we were able to thumbify.
For that, let's count the files we could resize as `1` and errors as `0`,
and sum them up:

```rust file=src/main.rs
    let thumbnail_count: i32 = thumbnails
        .map(|x| if x.is_ok() { 1 } else { 0 })
        .sum();
```

Sweet.
Now, let's print that number and we are done!

```rust file=src/main.rs
    println!(
        "{} of {} files successfully thumbyfied!",
        thumbnail_count,
        files.len()
    );
});
```

## Give it a spin!

1. Save the following images into a directory called `rust_memes`
   [1](https://i.imgur.com/S9ajSF3.jpg)
   [2](https://i.redd.it/dtne9fvkilsz.png)
   [3](https://i.redd.it/e3xrc6iqg4az.jpg)
2. `cargo run -- "rust_memes/*"`
3. ???
4. PROFIT!
