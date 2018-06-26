#![feature(plugin, custom_attribute)]
#![plugin(flamer)]
#![feature(associated_constants)]

extern crate byteorder;
extern crate clap;
extern crate flame;
extern crate histogram;
extern crate id3;
extern crate itertools;
extern crate memmap;
extern crate percent_encoding;
extern crate plist;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

extern crate regex;
#[macro_use]
extern crate lazy_static;

mod analysers;
mod comment_data;
mod input;
mod itunes;
mod shelltools;

use comment_data::BpmInfo;
use comment_data::CommentData;
// use input::audiobuffer::AudioBuffer;
use input::audiostream::AudioStream;

use analysers::bpmtools::BpmTools;

use shelltools::sox::*;

use itunes::library::Library;
use itunes::track::AudioFormat;

use histogram::Histogram;

use clap::{App, Arg};

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
        match &track.comment {
            Some(c) => {
                if track.audioformat == AudioFormat::Mp3 {
                    println!("Track: {}", track);
                    println!("\tComment: {:?}", c);
                    println!("\tId3 information: ");
                    // println!("\t(Reading from path: {:?})", track.location);
                    let tag = id3::Tag::read_from_path(&track.location).unwrap();
                    println!("\t\tId3artist: {}", tag.artist().unwrap());
                    // println!("{}", tag.comments().unwrap());

                    // build a commentdata from the track
                    let cd = CommentData {
                        algs: vec![BpmInfo {
                            bpm: 64.0,
                            alg: String::from("bpmish"),
                        }],
                    };

                    match cd.write_data(&track) {
                        Some(new_track) => {
                            let parsed_data = CommentData::parse_data(&new_track);

                            println!("Parsed data: {:?}", parsed_data);
                        }
                        None => println!("Could not parse ellington data section"),
                    };

                    // and parse it out again:
                }
            }
            None => {}
        }

        flame::start("streamed_call");
        let sox_stream = AudioStream::from_stream(SoxCall::default(&track.location).run());
        let calculated_bpm = BpmTools::default().analyse(sox_stream);
        flame::end("streamed_call");

        // let sox_stream = AudioStream::from_stream(SoxCall::default(track.location).run());
        // let calculated_bpm = BpmTools::default().analyse(sox_stream);

        if calculated_bpm != 0.0 {
            match bpm_hist.increment(calculated_bpm as u64) {
                _ => {}
            }
        }

        match track.bpm {
            Some(bpm) => {
                let error = percent_err(bpm as f64, calculated_bpm as f64);

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
        .arg(
            Arg::with_name("library")
                .short("l")
                .long("library")
                .value_name("library")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The iTunes library file with track information."),
        )
        .get_matches();

    let library_file = matches.value_of("library").unwrap();
    process_library(library_file);

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
