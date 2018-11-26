extern crate ellington;

// other crates to get stuff working.
extern crate assert_cmd;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;

use assert_cmd::prelude::*;
use std::process::Command;

use std::path::PathBuf;

lazy_static! {
    static ref DATA_DIRECTORY: PathBuf = {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test")
            .join("data");

        assert!(d.exists(), "Test data folder does not exist, failing.");

        d
    };
}

lazy_static! {
    static ref LIBRARY_FILE: PathBuf = {
        let d = DATA_DIRECTORY.join("lib").join("library.json");
        assert!(d.exists(), "Test library does not exist, failing.");
        d
    };
}

fn tmpdir() -> tempfile::TempDir {
    tempfile::Builder::new().tempdir_in("./").unwrap()
}

fn tmpfile() -> tempfile::NamedTempFile {
    tempfile::Builder::new().tempfile_in("./").unwrap()
}

#[test]
fn no_args() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().success();
    println!("Temp directory: {:?}", tmpdir().path().to_str());
}

#[test]
fn initialise_empty_library() {
    let tf = tmpfile();
    let mut cmd = Command::main_binary().args(&["init", "empty", ""]);
}

#[test]
fn initialise_fresh_library() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().success();
    println!("Data directory: {:?}", DATA_DIRECTORY.to_str());
    println!("LIBRARY FILE: {:?}", LIBRARY_FILE.to_str());
}
