#[macro_use] extern crate quicli;
use quicli::prelude::*;

main!({
    let x = read_file(".gitignore")?;
    println!("{}", x);
});
