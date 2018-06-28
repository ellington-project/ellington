#![feature(plugin, custom_attribute)]
#![plugin(flamer)]
#![feature(associated_constants)]

extern crate byteorder;

#[macro_use]
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
mod library;
mod profiling;
mod shelltools;

use profiling::Profile;
use std::io::Read;

use comment_data::BpmInfo;
use comment_data::CommentData;
// use input::audiobuffer::AudioBuffer;
use input::audiostream::AudioStream;

use analysers::bpmtools::BpmTools;

use shelltools::sox::*;

use library::library::Library;
use library::track::AudioFormat;

use histogram::Histogram;

use clap::App;

use std::fs::File;

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
        "\tPercentiles -- 5: {} 10: {} 15: {} 20: {} 25: {} 30: {} 35: {} 40: {} 45: {} 50: {} 55: {} 60: {} 65: {} 70: {} 75: {} 80: {} 85: {} 90: {} 95: {}",
        h.percentile(5.0).unwrap_or(9999999) as f64 / div,
        h.percentile(10.0).unwrap_or(9999999) as f64 / div,
        h.percentile(15.0).unwrap_or(9999999) as f64 / div,
        h.percentile(20.0).unwrap_or(9999999) as f64 / div,
        h.percentile(25.0).unwrap_or(9999999) as f64 / div,
        h.percentile(30.0).unwrap_or(9999999) as f64 / div,
        h.percentile(35.0).unwrap_or(9999999) as f64 / div,
        h.percentile(40.0).unwrap_or(9999999) as f64 / div,
        h.percentile(45.0).unwrap_or(9999999) as f64 / div,
        h.percentile(50.0).unwrap_or(9999999) as f64 / div,
        h.percentile(55.0).unwrap_or(9999999) as f64 / div,
        h.percentile(60.0).unwrap_or(9999999) as f64 / div,
        h.percentile(65.0).unwrap_or(9999999) as f64 / div,
        h.percentile(70.0).unwrap_or(9999999) as f64 / div,
        h.percentile(75.0).unwrap_or(9999999) as f64 / div,
        h.percentile(80.0).unwrap_or(9999999) as f64 / div,
        h.percentile(85.0).unwrap_or(9999999) as f64 / div,
        h.percentile(90.0).unwrap_or(9999999) as f64 / div,
        h.percentile(95.0).unwrap_or(9999999) as f64 / div,
    );
}

#[flame]
fn process_library(filename: &str) -> () {
    let library = Library::from_itunes_xml(filename).unwrap();

    // create a histogram:
    let mut error_hist = Histogram::new();
    let mut bpm_hist = Histogram::new();

    for track in library.tracks {
        flame::start("process_track");
        match (&track.comment, &track.bpm) {
            (Some(c), Some(b)) => {
                if track.audioformat == AudioFormat::Mp3 {
                    println!("Track: {}", track);
                    println!("Comment: {:?}", c);
                    println!("Id3 information: ");
                    // println!("\t(Reading from path: {:?})", track.location);
                    let tag = id3::Tag::read_from_path(&track.location).unwrap();
                    println!("Id3artist: {}", tag.artist().unwrap());
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

                    
                    let mut call = SoxCommand::default(&track.location);
                    let mut child = call.run();

                    {
                        
                        let sox_stream = match &mut child.stdout {
                            Some(s) => Some(AudioStream::from_stream(s)),
                            None => None,
                        }.unwrap();

                        let calculated_bpm = BpmTools::default().analyse(sox_stream);

                        

                        if calculated_bpm != 0.0 {
                            match bpm_hist.increment(calculated_bpm as u64) {
                                _ => {}
                            }
                        }

                        let error = percent_err(*b as f64, calculated_bpm as f64);

                        println!(
                            "calculated: {}, actual: {}, error: {}",
                            calculated_bpm, b, error
                        );

                        // get the error as an integer
                        let error_i = (error * 1000.0) as u64;
                        match error_hist.increment(error_i) {
                            _ => {}
                        };

                        println!("bpms:");
                        print_histogram(&bpm_hist, 1.0);
                        println!("errors:");
                        print_histogram(&error_hist, 1000.0);
                        println!("===== ===== ===== ===== =====");
                    }
                    child.wait().expect("failed to wait on child");
                }
            }
            _ => {
                println!("Ignore... {}", track.name);
            }
        }
        flame::end("process_track");
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let library_file = matches.value_of("library").unwrap();

    println!("Processing from library: {:?}", library_file);

    process_library(library_file);

    let profile = Profile::from_spans(flame::spans());
    profile.print();

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
