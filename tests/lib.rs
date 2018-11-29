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
        // Check to see that all the files exist.
        assert!(d.join("mp3").join("09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3").exists(), "Mp3s do not exist, failing!");
        assert!(d.join("flac").join("09 - Jumpin' at the Woodside - Count Basie And His Orchestra.flac").exists(), "Flacs do not exist, failing!");
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

#[allow(dead_code)]
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

    let gold_s = read_file(gold.to_str().unwrap().to_string());

    let ch = Changeset::new(computed_s.as_str(), gold_s.as_str(), "");

    if ch.distance == 0 {
        return true;
    } else {
        println!("Computed: {}", computed_s);
        println!("Gold: {}", gold_s);
        println!("Files differ: {}", ch);
        return false;
    }
}

mod general {
    use super::*;
    #[test]
    fn no_args() {
        Command::main_binary().unwrap().assert().success();
    }
}

mod library {
    use super::*;
    #[test]
    fn initialise_empty() {
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

    // Right now, this test doesn't work, as the libraries might be constructed in an unexpected order...
    #[test]
    #[ignore]
    fn initialise_fresh() {
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
}

mod query {
    use super::*;
    #[test]
    fn mp3_prepend_minimal_to_userdata() {
        let args = vec![
            "query", // The command to run.
            "resources/test/data/mp3/09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3", // The audiofile to query information about
            "resources/test/data/lib/library.json", // Specify a library to read data from
            "-m",                                   // Specify that we want to update the userdata
            "userdata",
            "-u", // Specify userdata to prepend to.
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
    fn mp3_update_minimial_userdata_pre() {
        let args = vec![
            "query", // The command to run.
            "resources/test/data/mp3/09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3", // The audiofile to query information about
            "resources/test/data/lib/library.json", // Specify a library to read data from
            "-m",                                   // Specify that we want to update the userdata
            "userdata",
            "-u", // Specify userdata to prepend to.
            "[ed|a~4242,n~4242,b~4242|] Test Userdata Data Pre On Command Line",
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
            .stdout("[ed|a~240,n~239,b~240|] Test Userdata Data Pre On Command Line\n");
    }

    #[test]
    fn mp3_update_minimial_userdata_post() {
        let args = vec![
            "query", // The command to run.
            "resources/test/data/mp3/09 - Jumpin' at the Woodside - Count Basie And His Orchestra.mp3", // The audiofile to query information about
            "resources/test/data/lib/library.json", // Specify a library to read data from
            "-m",                                   // Specify that we want to update the userdata
            "userdata",
            "-u", // Specify userdata to prepend to.
            "Test Userdata Data Post On Command Line [ed|a~4242,n~4242,b~4242|]",
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
            .stdout("Test Userdata Data Post On Command Line [ed|a~240,n~239,b~240|]\n");
    }

    #[test]
    fn mp3_prepend_minimal_to_title() {
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

    #[test]
    fn mp3_append_to_comments() {
        let args = vec![
            "query", // The command to run.
            "resources/test/data/mp3/Milenberg Joys - The 6-Alarm Six - Rappolo.mp3", // The audiofile to query information about
            "resources/test/data/lib/library.json", // Specify a library to read data from
            "-m",                                   // Specify that we want to update the userdata
            "comments",
            "-o", // Specify that we want to update the userdata.
            "update",
            "-b", // Specify that we want to append to the userdata
            "append",
            //"-a", // report minimally
            "-n", // don't run estimators
            "-p", // don't modify the library
        ];

        println!("Args: {}", args.join(" "));

        Command::main_binary()
        .unwrap()
        .args(&args)
        .assert()
        .success()
        .stdout("https://archive.org/details/78_milenberg-joys_the-6-alarm-six-rappolo-jellyroll-morton-mares_gbia0001104a [ed| actual~230, naive~230, bellson~230 |]\n");
    }
}
