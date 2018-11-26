extern crate assert_cmd;
extern crate ellington;
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

        assert!(d.exists(), "Folder containing test data does not exist!");

        d
    };
}

lazy_static! {
    static ref LIBRARY_FILE: PathBuf = {
        let d = DATA_DIRECTORY.join("lib").join("library.json");
        assert!(d.exists(), "Library of test data does not exist");
        d
    };
}

#[test]
fn no_args() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().success();
    println!("Data directory: {:?}", DATA_DIRECTORY.to_str());
    println!("LIBRARY FILE: {:?}", LIBRARY_FILE.to_str());
}

#[test]
fn simple_query() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().success();
    println!("Data directory: {:?}", DATA_DIRECTORY.to_str());
    println!("LIBRARY FILE: {:?}", LIBRARY_FILE.to_str());
}
