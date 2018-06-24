#![feature(plugin, custom_attribute)]
#![plugin(flamer)]

extern crate flame;
extern crate rand;
extern crate byteorder;
extern crate histogram;
extern crate memmap;
extern crate plist;
extern crate clap;

mod audio_in;
mod itunes;
mod shelltools;
mod analysers;

use audio_in::AudioBuffer;
use analysers::bpmtools::BpmTools;

use shelltools::sox::call_sox_and_read_f32;

use itunes::library::Library;

use shelltools::bpm::bpm_track;

use histogram::Histogram;

use clap::{Arg, App};

use std::fs::File;



#[flame]
fn percent_err(gold: f64, trial: f64) -> f64 {
    return ((gold - trial).abs() / gold) * 100.0;
}

#[flame]
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

#[flame]
fn process_library(filename: &str) -> () {

    let library = Library::from_filename(filename).unwrap();

    // create a histogram:
    let mut error_hist = Histogram::new();
    let mut bpm_hist = Histogram::new();

    for track in library.tracks {
        println!("Track: {}", track);
        let calculated_bpm = bpm_track(&track).unwrap_or(0.0);

        let AudioBuffer(sox_data) = call_sox_and_read_f32(&track);

        let rust_bpm = BpmTools::default().analyse(&sox_data);

        println!("Calculate the bpm as {} with rust, compared to {} with C", rust_bpm, calculated_bpm);

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
                println!("calculated: {}, actual: -, error: -", calculated_bpm);
            }
        }
        println!("bpms:");
        print_histogram(&bpm_hist, 1.0);
        println!("errors:");
        print_histogram(&error_hist, 1000.0);
    }
}

fn main() {
    let matches = App::new("ellington")
        .version("0.1.0")
        .author("Adam Harries <harries.adam@gmail.com>")
        .about("Automated BPM calculation for swing dance DJs")
        .arg(Arg::with_name("library")
                .short("l")
                .long("library")
                .value_name("library")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The iTunes library file with track information.")) 
        .get_matches();

    let library_file = matches.value_of("library").unwrap();
    process_library(library_file);

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    // process_library("res/partialLibrary.xml");
    // process_library("/Users/adam/Music/iTunes/iTunes Music Library.xml");
    // process_library("/Users/adam/Music/iTunes/iTunes Music Library.xml");
}
