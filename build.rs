#[macro_use]
extern crate commandspec;
use commandspec::*; // .execute() method on Command

use std::path::PathBuf;
use std::process::Command;
use std::env; 

#[cfg(target_os = "linux")]
fn print_linker_arguments() {
    println!("Printing linker arguments for Linux");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-flags=-l tag_c -l tag -l z");
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
        // "https://archive.org/download/78_crazy-rhythm_coleman-hawkins-and-his-all-star-jam-band-coleman-hawkins-alix-combe_gbia0004564b/Crazy%20Rhythm%20-%20Coleman%20Hawkins%20and%20his%20All-Star%20%22Jam%22%20Band.flac",
        // "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/01%20-%20One%20O%27Clock%20Jump%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/09%20-%20Jumpin%27%20at%20the%20Woodside%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        // "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/12%20-%20Jive%20at%20Five%20-%20Count%20Basie%20And%20His%20Orchestra.flac",
        // "https://archive.org/download/78_the-little-man-who-wasnt-there_glenn-miller-and-his-orchestra-harold-adamson-berna_gbia0015208b/The%20Little%20Man%20Who%20Wasn%27t%20-%20Glenn%20Miller%20and%20his%20Orchestra.flac",
        // "https://archive.org/download/78_rag-mop_johnnie-lee-wills-and-his-boys-anderson-wills_gbia0002933a/Rag%20Mop%20-%20Johnnie%20Lee%20Wills%20and%20His%20Boys.flac",
        // "https://archive.org/download/78_the-devil-aint-lazy_bob-wills-and-his-texas-playboys-fred-rose-tommy-duncan_gbia0007445a/The%20Devil%20Ain%27t%20Lazy%20-%20Bob%20Wills%20and%20his%20Texas%20Playboys.flac",
        // "https://archive.org/download/78_song-of-the-volga-boatman_ray-brown-trio-ray-brown-buddy-rich-hank-jones-norman-gra_gbia0009677b/Song%20of%20The%20Volga%20Boatman%20-%20Ray%20Brown%20Trio.flac",
        // "https://archive.org/download/78_when-the-saints-go-marching-in_pete-dailys-dixieland-band-pete-daily-warren-smith_gbia0001062b/When%20The%20Saints%20Go%20Marching%20In%20-%20Pete%20Daily%27s%20Dixieland%20Band.flac",
        "https://archive.org/download/78_milenberg-joys_the-6-alarm-six-rappolo-jellyroll-morton-mares_gbia0001104a/Milenberg%20Joys%20-%20The%206-Alarm%20Six%20-%20Rappolo.flac"
    ];

    let mp3_urls = vec![
        "https://archive.org/download/78_countless-blues_the-kansas-city-six-eddie-druham-freddie-green-walter-paige-joe-jon_gbia0004728a/Countless%20Blues%20-%20The%20Kansas%20City%20Six%20-%20Eddie%20Druham.mp3",
        // "https://archive.org/download/78_crazy-rhythm_coleman-hawkins-and-his-all-star-jam-band-coleman-hawkins-alix-combe_gbia0004564b/Crazy%20Rhythm%20-%20Coleman%20Hawkins%20and%20his%20All-Star%20%22Jam%22%20Band.mp3",
        // "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/01%20-%20One%20O%27Clock%20Jump%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/09%20-%20Jumpin%27%20at%20the%20Woodside%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        // "https://archive.org/download/78_every-tub_count-basie-and-his-orchestra-count-basie_gbia0032723/12%20-%20Jive%20at%20Five%20-%20Count%20Basie%20And%20His%20Orchestra.mp3",
        // "https://archive.org/download/78_the-little-man-who-wasnt-there_glenn-miller-and-his-orchestra-harold-adamson-berna_gbia0015208b/The%20Little%20Man%20Who%20Wasn%27t%20-%20Glenn%20Miller%20and%20his%20Orchestra.mp3",
        // "https://archive.org/download/78_rag-mop_johnnie-lee-wills-and-his-boys-anderson-wills_gbia0002933a/Rag%20Mop%20-%20Johnnie%20Lee%20Wills%20and%20His%20Boys.mp3",
        // "https://archive.org/download/78_the-devil-aint-lazy_bob-wills-and-his-texas-playboys-fred-rose-tommy-duncan_gbia0007445a/The%20Devil%20Ain%27t%20Lazy%20-%20Bob%20Wills%20and%20his%20Texas%20Playboys.mp3",
        // "https://archive.org/download/78_song-of-the-volga-boatman_ray-brown-trio-ray-brown-buddy-rich-hank-jones-norman-gra_gbia0009677b/Song%20of%20The%20Volga%20Boatman%20-%20Ray%20Brown%20Trio.mp3",
        // "https://archive.org/download/78_when-the-saints-go-marching-in_pete-dailys-dixieland-band-pete-daily-warren-smith_gbia0001062b/When%20The%20Saints%20Go%20Marching%20In%20-%20Pete%20Daily%27s%20Dixieland%20Band.mp3",
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
            println!("Downloading to folder: {}", testdata_folder.to_str().unwrap());
            // Download data for our tests. 

            let mut mp3_folder = testdata_folder.clone();
            mp3_folder.push("mp3");            
            for url in mp3_urls { 
                download(&url.to_string(), &mp3_folder);
            }

            let mut flac_folder = testdata_folder.clone();
            flac_folder.push("flac");            
            for url in flac_urls { 
                download(&url.to_string(), &flac_folder);
            }
        }, 
        // else
        Err(_) => { 
            println!("Did not detect data driven tests.");
        }
    };
}

fn download(url: &String, path: &PathBuf) -> () { 
    println!("Downloading: {}", url );

    sh_execute!(
        r#"
            echo "Shell downloading {url} to {path}";
            mkdir -p {path};
            echo "Made path {path}"; 
            wget -P {path} {url};
        "#,
        url = url.clone(), 
        path = path.to_str().unwrap(),
    ); 
    // {
    //     Ok(c) => println!("Ran command {} successfully", c), 
    //     Err(c) => println!("Got error {:?} while running!", c)
    // };
}