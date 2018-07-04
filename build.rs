use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Download some data for our tests.
    let testdata_url =
        "https://archive.org/download/78_little-brown-jug_glenn-miller-and-his-orchestra-glenn-miller_gbia0015205a/Little%20Brown%20Jug%20-%20Glenn%20Miller%20and%20his%20Orchestra.mp3";

    let mut testdata_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testdata_folder.push("data");

    let mut testdata_mp3 = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testdata_mp3.push("data/test.mp3");

    let mut testdata_raw = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testdata_raw.push("data/test.raw");

    if !testdata_raw.as_path().exists() {
        // make a folder for our testdata
        // let output = Command::new("mkdir")
        //     .arg("-p")
        //     .arg(testdata_folder.to_str().unwrap())
        //     .output()
        //     .expect("Failed to make testdata directory!");
        // assert!(output.status.success());

        // // download the data to that folder.
        // let output = Command::new("wget")
        //     .arg(testdata_url)
        //     .arg("-O")
        //     .arg(testdata_mp3.to_str().unwrap())
        //     .output()
        //     .expect("Failed to download test data");
        // assert!(output.status.success());

        // // finally make some raw data
        // let output = Command::new("sox")
        //     .arg("-V1")
        //     .arg(testdata_mp3)
        //     .arg("-r")
        //     .arg("44100")
        //     .arg("-e")
        //     .arg("float")
        //     .arg("-c")
        //     .arg("1")
        //     .arg("-b")
        //     .arg("16")
        //     .arg("-t")
        //     .arg("raw")
        //     .arg(testdata_raw)
        //     .output()
        //     .expect("Failed to run sox command successfully");

        // assert!(output.status.success());
    }
}
