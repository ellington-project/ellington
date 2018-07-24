#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! clap = "2.32.0"
//! toml = "0.4"
//! ```
extern crate clap;
extern crate time;
extern crate toml;

use clap::{App, Arg};
use std::env;
fn main() {
    let matches = App::new("package")
        .version("1.0")
        .author("Adam Harries <harries.adam@gmail.com>")
        .about("Package up a rust crate into a zip for easy redistribution to non-technical users")
        .arg(
            Arg::with_name("manifest")
                .help("The manifest file with ")
                .index(1),
        )
        .get_matches();

        if let Some(m) = matches.value_of("manifest") {
        println!("Value for manifest: {}", m);
}
    println!("{}", time::now().rfc822z());

}
