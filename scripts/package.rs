#!/usr/bin/env run-cargo-script
//!
//! package.rs - generic packaging for ellington, across windows mac and linux.
//!
//! usage: `cargo script -- scripts/package.rs <ellington directory> <ellington tag>
//!
//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! clap = "2.32.0"
//! toml = "0.4"
//! walkdir = "2"
//! ```
extern crate clap;
extern crate time;
extern crate toml;
extern crate walkdir;

use clap::{App, Arg};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    // Parse arguments etc.
    let matches = App::new("package")
        .version("1.0")
        .author("Adam Harries <harries.adam@gmail.com>")
        .about("Package up a rust crate into a zip for easy redistribution to non-technical users")
        .arg(
            Arg::with_name("ellington_root")
                .help("The root directory of the ellington project")
                .index(1),
        ).arg(
            Arg::with_name("tag")
                .help("The tag that we're building to incorporate into the release name")
                .index(2),
        ).get_matches();

    let elrt = matches.value_of("ellington_root").unwrap_or("./");
    println!("Packaging ellington based on root: {}", elrt);

    let tag = matches.value_of("tag").unwrap_or("untagged");
    println!("Labelling build/package with tag: {}", tag);

    // We're assuming we're building in release mode
    let build_dir = Path::new(&elrt)
        .join("target")
        .join("release")
        .join("build");

    // Build up a list of libraries that we need to package with the main executable
    // Use Rust's cfg! macros to pick which libraries we want to package based on the
    // operating system for which we're packaging
    let libs: Vec<&'static str> = libs();
    let mut libmap: HashMap<String, PathBuf> = HashMap::new();

    

    // create the package directory
    // fs::create_dir("release-");

    // Walk the build directory to search for the libraries that we need.
    for entry in WalkDir::new(build_dir).contents_first(true) {
        match entry {
            Ok(path) => {
                let name = path.file_name().to_str().unwrap();

                for libname in &libs {
                    if *libname == name {
                        println!("Found shared library: {:?}", name);
                        match libmap.insert(name.into(), path.path().into()) {
                            None => println!("Found duplicate of library {:?}", name),
                            _ => {}
                        };
                    }
                }
            }
            Err(e) => {
                println!("While walking build dir, got error: {:?}", e);
            }
        };
    }

    // iterate over the located libraries, and copy them to a package directory
    for (key, va) in libmap.iter() { 
        println!("Lib: {:?}, \n\tpath: {:?}", key, va);
    }    
}

fn libs() -> Vec<&'static str> { 
    if cfg!(target_os = "windows") {
        return vec!("libtag.dll");
    }
    if cfg!(target_os = "macos") {
        return vec!("libtag.dylib");
    }
    if cfg!(target_os = "linux") {
        return vec!("libtag.so", "libtag.so.1", "libtag.so.1.17.0");
    }
    panic!("No list of libraries given for this platform!");
    return vec!();
}
