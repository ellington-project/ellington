extern crate ellington;

// other crates to get stuff working.
extern crate assert_cmd;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;
extern crate difference;

use difference::Changeset;

use assert_cmd::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

lazy_static! {
    static ref test_resources_dir: PathBuf = {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test");
        assert!(d.exists(), "Test resources folder does not exist, failing.");
        d
    };
}

lazy_static! {
    static ref test_data_dir: PathBuf = {
        let d = test_resources_dir.join("data");
        assert!(d.exists(), "Test data folder does not exist, failing.");
        d
    };
}

lazy_static! {
    static ref test_gold_dir: PathBuf = {
        let d = test_resources_dir.join("gold");
        assert!(d.exists(), "Test gold folder does not exist, failing.");
        d
    };
}

fn tmpdir() -> tempfile::TempDir {
    let dir = tempfile::Builder::new().tempdir_in("./").unwrap();
    println!("Initialised temp dir: {}", dir.path().to_str().unwrap());
    dir
}

fn tmpfile() -> tempfile::NamedTempFile {
    let file = tempfile::Builder::new().tempfile_in("./").unwrap();
    println!("Initialised temp file: {}", file.path().to_str().unwrap());
    file
}

fn read_file(filename: String) -> String {
    println!("Reading file contents from: {}", filename);
    let mut f = File::open(filename).expect("File not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading file!");
    contents
}

fn diff_files(computed: &PathBuf, gold: &PathBuf) -> bool {
    // read the two files to strings.
    let computed_s = read_file(computed.to_str().unwrap().to_string());
    // println!("Computed string:\n{}", computed_s);

    let gold_s = read_file(gold.to_str().unwrap().to_string());
    // println!("Gold string:\n{}", gold_s);

    let ch = Changeset::new(computed_s.as_str(), gold_s.as_str(), "");

    if ch.distance == 0 {
        return true;
    } else {
        println!("Files differ: {}", ch);
        return false;
    }
}

#[test]
fn no_args() {
    Command::main_binary().unwrap().assert().success();
}

#[test]
fn initialise_empty_library() {
    // Initialise a tempfile for the lib.
    let tf = tmpfile();
    // Run the command, assert that it's successful.
    Command::main_binary()
        .unwrap()
        .args(&["init", "empty", tf.path().to_str().unwrap()])
        .assert()
        .success();

    // Diff the computed and gold.
    assert!(diff_files(
        &tf.path().to_path_buf(),
        &test_gold_dir.join("empty.json")
    ));
}

#[test]
fn initialise_fresh_library() {
    // Initialise a tempfile for the lib
    let tf = tmpfile();

    // Run the command, assert that it's successful.
    Command::main_binary()
        .unwrap()
        .args(&[
            "init",
            "directory",
            tf.path().to_str().unwrap(),
            "-d",
            "resources/test/data", // we have to use the relative path
        ])
        .assert()
        .success();

    // Diff the computed and gold.
    assert!(diff_files(
        &tf.path().to_path_buf(),
        &test_gold_dir.join("fresh_library.json")
    ));
}

#[test]
fn query_and_append_to_userdata() {
    // Run the command, and get the output.
    let args = vec![
        "query", // The command to run.
        "resources/test/data/mp3/09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3", // The audiofile to query information about
        "resources/test/data/lib/library.json", // Specify a library to read data from
        "-m",                                   // Specify that we want to update the userdata
        "userdata",
        "-u", // Specify userdata to append to.
        "Test Userdata On Command Line",
        "-o", // Specify that we want to update the userdata.
        "update",
        "-a", // report minimally
        "-n", // don't run estimators
        "-p", // don't modify the library
    ];

    println!("Args: {}", args.join(" "));

    Command::main_binary()
        .unwrap()
        .args(&args)
        .assert()
        .success()
        .stdout("[ed|a~240,n~239,b~240|] Test Userdata On Command Line\n");
}

#[test]
fn query_and_append_to_title() {
    // Run the command, and get the output.
    let args = vec![
        "query", // The command to run.
        "resources/test/data/mp3/09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3", // The audiofile to query information about
        "resources/test/data/lib/library.json", // Specify a library to read data from
        "-m",                                   // Specify that we want to update the userdata
        "title",
        "-o", // Specify that we want to update the userdata.
        "update",
        "-a", // report minimally
        "-n", // don't run estimators
        "-p", // don't modify the library
    ];

    println!("Args: {}", args.join(" "));

    Command::main_binary()
        .unwrap()
        .args(&args)
        .assert()
        .success()
        .stdout("[ed|a~240,n~239,b~240|] Jumpin' at the Woodside\n");
}
