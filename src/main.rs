extern crate histogram;
extern crate plist;
extern crate byteorder;
extern crate memmap;

mod track;
mod audio_in;
mod itunes;

use std::fs::File;
use std::num::ParseFloatError;

use std::process::Command;

use plist::Plist;

use histogram::Histogram;

use track::Track;

use itunes::*;

fn extract_track(trackpl: &plist::Plist) -> Option<Track> {
    // assert the track plist is a dictionary
    let trackinfo = trackpl.as_dictionary()?;

    // build a track with information extracted from the dict
    // bail out (and return None) if we fail to get any of:
    // - track id
    // - name
    // - location
    // fill the BPM with "none" if no bpm found
    Some(Track {
        itunes_id: trackinfo.get("Track ID")?.as_integer()?,
        bpm: trackinfo.get("BPM").and_then(|b| b.as_integer()),
        name: trackinfo.get("Name")?.as_string()?.to_string(),
        location: trackinfo.get("Location")?.as_string()?.to_string(),
    })
}

fn bpm_track(track: &Track) -> Result<f64, ParseFloatError> {
    let command = format!(
        "tools/bpm-tools/bpm-print -e tools/bpm-tools/bpm -m 10 -x 500 \"{}\"",
        track.location.replace("%20", " ").replace("file://", "")
    );

    let res = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    // let errors = String::from_utf8_lossy(&res.stderr);

    let result = String::from_utf8_lossy(&res.stdout).replace("\n", "");
    // parse the result into an f64, discarding accuracy.

    // return match result {
    //     Ok(f) => Some(f),
    //     Err(e) => {println!("Got error {:?} wile parsing", e); None}
    // };
    return result.parse::<f64>();
}

fn percent_err(gold: f64, trial: f64) -> f64 {
    return ((gold - trial).abs() / gold) * 100.0;
}

fn print_histogram(h: &Histogram, div: f64) -> () {
    println!(
        "\tRunning stats -- Min: {} Avg: {} Max: {} StdDev: {} ",
        h.minimum().unwrap_or(9999999) as f64 / div,
        h.mean().unwrap_or(9999999) as f64 / div,
        h.maximum().unwrap_or(9999999) as f64 / div,
        h.stddev().unwrap_or(9999999) as f64 / div
    );

    println!(
        "\tPercentiles -- 5: {} 10: {} 25: {} 50: {} 75: {} 90: {} 95: {}",
        h.percentile(5.0).unwrap_or(9999999) as f64 / div,
        h.percentile(10.0).unwrap_or(9999999) as f64 / div,
        h.percentile(25.0).unwrap_or(9999999) as f64 / div,
        h.percentile(50.0).unwrap_or(9999999) as f64 / div,
        h.percentile(75.0).unwrap_or(9999999) as f64 / div,
        h.percentile(90.0).unwrap_or(9999999) as f64 / div,
        h.percentile(95.0).unwrap_or(9999999) as f64 / div
    );
}

fn process_library(filename: &str) -> () {
    let file = File::open(filename).unwrap();

    let plist = Plist::read(file).unwrap();

    // get the tracks from the PList:
    let tracks = plist.as_dictionary().unwrap().get("Tracks").unwrap();

    println!(
        "Found {} tracks in the tracklist",
        tracks.as_dictionary().unwrap().len()
    );

    let tracks = tracks.as_dictionary().unwrap().values().map(extract_track);

    // create a histogram:
    let mut error_hist = Histogram::new();
    let mut bpm_hist = Histogram::new();

    for t in tracks {
        match t {
            Some(track) => {
                println!("Track: {}", track);
                let calculated_bpm = bpm_track(&track).unwrap_or(0.0);
                if calculated_bpm != 0.0 {
                    match bpm_hist.increment(calculated_bpm as u64) {
                        _ => {}
                    }
                }
                match track.bpm {
                    Some(bpm) => {
                        let error = percent_err(bpm as f64, calculated_bpm);
                        println!(
                            "calculated: {}, actual: {}, error: {}",
                            calculated_bpm, bpm, error
                        );

                        // get the error as an integer
                        let error_i = (error * 1000.0) as u64;
                        match error_hist.increment(error_i) {
                            _ => {}
                        };
                        
                    }
                    None => {
                        println!(
                            "calculated: {}, actual: -, error: -",
                            calculated_bpm
                        );
                    }
                }
                println!("bpms:");
                print_histogram(&bpm_hist, 1.0);
                println!("errors:");
                print_histogram(&error_hist, 1000.0);
            }
            None => println!("Got bad track."),
        }
        println!("   ---")
    }
}

fn main() {
    // process_library("res/partialLibrary.xml");
    process_library("/Users/adam/Music/iTunes/iTunes Music Library.xml");
    // println!("Hello, world!");
}
