#[macro_use]
extern crate commandspec;
use commandspec::*; // .execute() method on Command

use std::path::PathBuf;
use std::process::Command;
use std::env; 

#[cfg(target_os = "linux")]
fn print_linker_arguments() {
    println!("Printing linker arguments for Linux");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=tag_c");
    println!("cargo:rustc-link-lib=static=stdc++");
}

#[cfg(target_os = "macos")]
fn print_linker_arguments() {
    println!("Printing linker arguments for OSX");
    println!("cargo:rustc-flags=-l dylib=z");
    println!("cargo:rustc-flags=-l tag_c -l tag");
    println!("cargo:rustc-link-lib=c++");
}

fn main() {
    print_linker_arguments();

    let flac_urls = vec![
        "https://archive.org/download/78_countless-blues_the-kansas-city-six-eddie-druham-freddie-green-walter-paige-joe-jon_gbia0004728a/Countless%20Blues%20-%20The%20Kansas%20City%20Six%20-%20Eddie%20Druham.flac",
        "https://archive.org/download/78_crazy-rhythm_coleman-hawkins-and-his-all-star-jam-band-coleman-hawkins-alix-combe_gbia0004564b/Crazy%20Rhythm%20-%20Coleman%20Hawkins%20and%20his%20All-Star%20%22Jam%22%20Band.flac",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/01%20-%20One%20O%27Clock%20Jump%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/09%20-%20Jumpin%27%20at%20the%20Woodside%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/12%20-%20Jive%20at%20Five%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        "https://archive.org/download/78_the-little-man-who-wasnt-there_glenn-miller-and-his-orchestra-harold-adamson-berna_gbia0015208b/The%20Little%20Man%20Who%20Wasn%27t%20-%20Glenn%20Miller%20and%20his%20Orchestra.flac",
        "https://archive.org/download/78_rag-mop_johnnie-lee-wills-and-his-boys-anderson-wills_gbia0002933a/Rag%20Mop%20-%20Johnnie%20Lee%20Wills%20and%20His%20Boys.flac",
        "https://archive.org/download/78_the-devil-aint-lazy_bob-wills-and-his-texas-playboys-fred-rose-tommy-duncan_gbia0007445a/The%20Devil%20Ain%27t%20Lazy%20-%20Bob%20Wills%20and%20his%20Texas%20Playboys.flac",
        "https://archive.org/download/78_song-of-the-volga-boatman_ray-brown-trio-ray-brown-buddy-rich-hank-jones-norman-gra_gbia0009677b/Song%20of%20The%20Volga%20Boatman%20-%20Ray%20Brown%20Trio.flac",
        "https://archive.org/download/78_when-the-saints-go-marching-in_pete-dailys-dixieland-band-pete-daily-warren-smith_gbia0001062b/When%20The%20Saints%20Go%20Marching%20In%20-%20Pete%20Daily%27s%20Dixieland%20Band.flac",
        "https://archive.org/download/78_milenberg-joys_the-6-alarm-six-rappolo-jellyroll-morton-mares_gbia0001104a/Milenberg%20Joys%20-%20The%206-Alarm%20Six%20-%20Rappolo.flac"
    ];

    let mp3_urls = vec![
        "https://archive.org/download/78_countless-blues_the-kansas-city-six-eddie-druham-freddie-green-walter-paige-joe-jon_gbia0004728a/Countless%20Blues%20-%20The%20Kansas%20City%20Six%20-%20Eddie%20Druham.mp3",
        "https://archive.org/download/78_crazy-rhythm_coleman-hawkins-and-his-all-star-jam-band-coleman-hawkins-alix-combe_gbia0004564b/Crazy%20Rhythm%20-%20Coleman%20Hawkins%20and%20his%20All-Star%20%22Jam%22%20Band.mp3",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/01%20-%20One%20O%27Clock%20Jump%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/09%20-%20Jumpin%27%20at%20the%20Woodside%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/12%20-%20Jive%20at%20Five%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        "https://archive.org/download/78_the-little-man-who-wasnt-there_glenn-miller-and-his-orchestra-harold-adamson-berna_gbia0015208b/The%20Little%20Man%20Who%20Wasn%27t%20-%20Glenn%20Miller%20and%20his%20Orchestra.mp3",
        "https://archive.org/download/78_rag-mop_johnnie-lee-wills-and-his-boys-anderson-wills_gbia0002933a/Rag%20Mop%20-%20Johnnie%20Lee%20Wills%20and%20His%20Boys.mp3",
        "https://archive.org/download/78_the-devil-aint-lazy_bob-wills-and-his-texas-playboys-fred-rose-tommy-duncan_gbia0007445a/The%20Devil%20Ain%27t%20Lazy%20-%20Bob%20Wills%20and%20his%20Texas%20Playboys.mp3",
        "https://archive.org/download/78_song-of-the-volga-boatman_ray-brown-trio-ray-brown-buddy-rich-hank-jones-norman-gra_gbia0009677b/Song%20of%20The%20Volga%20Boatman%20-%20Ray%20Brown%20Trio.mp3",
        "https://archive.org/download/78_when-the-saints-go-marching-in_pete-dailys-dixieland-band-pete-daily-warren-smith_gbia0001062b/When%20The%20Saints%20Go%20Marching%20In%20-%20Pete%20Daily%27s%20Dixieland%20Band.mp3",
        "https://archive.org/download/78_milenberg-joys_the-6-alarm-six-rappolo-jellyroll-morton-mares_gbia0001104a/Milenberg%20Joys%20-%20The%206-Alarm%20Six%20-%20Rappolo.mp3"
    ];
    // Query the CFG option to see if we want to do data-based tests. 
    let data_driven_tests = env::var("CARGO_FEATURE_DATA_DRIVEN_TESTS");
    match data_driven_tests  {
        // if it's set
        Ok(_) => { 
            // Create a folder to store test data
            let mut testdata_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            testdata_folder.push("data");
            // Download data for our tests. 
            for url in flac_urls { 
                download(&url.to_string(), &testdata_folder);
            }
        }, 
        // else
        Err(_) => { 
            println!("Did not detect data driven tests.");
        }
    };

    

    // Download some data for our tests.
    // let testdata_url =
    //     "https://archive.org/download/78_little-brown-jug_glenn-miller-and-his-orchestra-glenn-miller_gbia0015205a/Little%20Brown%20Jug%20-%20Glenn%20Miller%20and%20his%20Orchestra.mp3";

    // let mut testdata_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // testdata_folder.push("data");

    // let mut testdata_mp3 = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // testdata_mp3.push("data/test.mp3");

    // let mut testdata_raw = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // testdata_raw.push("data/test.raw");

    // if !testdata_raw.as_path().exists() {
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
    // }
}

fn download(url: &String, path: &PathBuf) -> () { 
    println!("Downloading: {}", url );
    execute!(
        "echo \"Hello from {url} world to {path}\"", 
        url = url.clone(), 
        path = path.to_str(),
    );
}