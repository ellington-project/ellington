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
use std::fs;
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

    let package_name = release_name(tag);
    let package_directory = Path::new(&elrt).join(&package_name);
    println!("Package name: {:?}", package_name);

    // create the package directory
    match fs::create_dir(&package_directory) {
        Ok(_) => {}
        Err(_) => {
            // try again - it's probably just there already!
            // at this point we'd rather just panic if it goes wrong
            // as we can't really continue from there!
            fs::remove_dir_all(&package_directory).unwrap();
            fs::create_dir(&package_directory).unwrap();
        }
    };

    // We're assuming we're building in release mode
    let build_dir = Path::new(&elrt).join("target").join("release");

    // create a map between files that we need, and their location on the disk.
    let mut filemap: HashMap<String, PathBuf> = HashMap::new();

    // The main ellington executable
    if cfg!(target_os = "windows") {
        filemap.insert(
            "ellington.exe".into(),
            build_dir.clone().join("ellington.exe"),
        );
    } else {
        filemap.insert("ellington".into(), build_dir.clone().join("ellington"));
    }

    // Documentation
    filemap.insert("README.md".into(), Path::new(&elrt).join("README.md"));
    // License
    filemap.insert("LICENSE.md".into(), Path::new(&elrt).join("LICENSE.txt"));

    // Build up a list of libraries that we need to package with the main executable
    // Use Rust's cfg! macros to pick which libraries we want to package based on the
    // operating system for which we're packaging
    let libs: Vec<&'static str> = libs();

    // Walk the build directory to search for the libraries that we need.
    for entry in WalkDir::new(build_dir.join("build")).contents_first(true) {
        match entry {
            Ok(path) => {
                let name = path.file_name().to_str().unwrap();
                for libname in &libs {
                    if *libname == name {
                        println!("Found shared library: {:?}", name);
                        match filemap.insert(name.into(), path.path().into()) {
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
    for (filename, path) in filemap.iter() {
        let source_path = path;
        let dest_path = package_directory.join(filename);
        println!(
            "Copying file \n\t{:?}\nto package file \n\t{:?}",
            source_path, dest_path
        );
        match std::fs::copy(&source_path, dest_path) {
            Ok(b) => {
                println!("Successfully copied {:?} bytes.", b);
            }
            Err(e) => {
                println!("Encountered error: {:?} while copying.", e);
            }
        };
    }
}

fn release_name(tag: &str) -> String {
    if cfg!(target_os = "windows") {
        return format!("ellington-windows-{}", tag);
    }
    if cfg!(target_os = "macos") {
        return format!("ellington-osx-{}", tag);
    }
    if cfg!(target_os = "linux") {
        return format!("ellington-linux-{}", tag);
    }
    // panic!("No list of libraries given for this platform!");
    return format!("unknown-untagged");
}

fn libs() -> Vec<&'static str> {
    if cfg!(target_os = "windows") {
        return vec!["libtag.dll"];
    }
    if cfg!(target_os = "macos") {
        return vec!["libtag.dylib"];
    }
    if cfg!(target_os = "linux") {
        return vec!["libtag.so", "libtag.so.1", "libtag.so.1.17.0"];
    }
    panic!("No list of libraries given for this platform!");
    return vec![];
}
