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
mod input;
mod library;
mod profiling;
mod shelltools;

use clap::ArgMatches;
use profiling::Profile;

use library::ellingtondata::BpmInfo;
use library::ellingtondata::EllingtonData;
// use input::audiobuffer::AudioBuffer;
use input::audiostream::AudioStream;

use analysers::bpmtools::BpmTools;

use shelltools::sox::*;

use library::library::Library;

use histogram::Histogram;

use clap::App;

use std::fs::File;

#[flame]
fn percent_err(gold: f64, trial: f64) -> f64 {
    return ((gold - trial).abs() / gold) * 100.0;
}

#[flame]
fn print_histogram(h: &Histogram, div: f64) -> () {
    println!(
        "Percentiles -- 75: {} 80: {} 85: {} 90: {} 95: {}",
        h.percentile(75.0).unwrap_or(9999999) as f64 / div,
        h.percentile(80.0).unwrap_or(9999999) as f64 / div,
        h.percentile(85.0).unwrap_or(9999999) as f64 / div,
        h.percentile(90.0).unwrap_or(9999999) as f64 / div,
        h.percentile(95.0).unwrap_or(9999999) as f64 / div,
    );
}

#[flame]
fn process_library(filename: &str) -> () {

    // Parse the library from an itunes file
    let library = Library::from_itunes_xml(filename).unwrap();

    println!("Successfully parsed {} tracks.", library.tracks.len());
    
    let mut error_hist = Histogram::new();

    // Iterate over the tracks.
    for track in library.tracks {
        flame::start("process_track");

        // Match the tracks that contain ellington data
        match track.ellington_data() {
            // If we have ellington data
            Some(ed) => {
                println!("Track: {}", track);
                println!("Bpm: {:?}", track.bpm());
                println!("Comment: {:#?}", track.comments());
                println!("Ed: {:#?}", ed);

                let mut call = SoxCommand::default(&track.location());
                let mut child = call.run();

                let cbpm = {
                    let sox_stream = match &mut child.stdout {
                        Some(s) => Some(AudioStream::from_stream(s)),
                        None => None,
                    }.unwrap();

                    let calculated_bpm = 
                        BpmTools::default().analyse(sox_stream);

                    calculated_bpm
                };

                child.wait().expect("failed to wait on child");

                println!("Calculated bpm: {}", cbpm);

                // build some ellington data 
                let new_data = EllingtonData { 
                    algs: Some (
                        vec![BpmInfo{
                            bpm: cbpm as i64, 
                            alg: "naive".to_string()
                        }]
                    )
                };

                match track.write_data(new_data) {
                    Some(_) => println!("Successfully written data."), 
                    None => println!("Failed to write id3 data for some reason.")
                }
                
                println!("===== ===== ===== ===== =====\n");
            }
            _ => {
                println!("Ignore... {:?}", track.name());
            }
        }

        flame::end("process_track");
    }
}

#[flame]
fn dispatch(matches: ArgMatches) -> () {
    // first hope that we have an iTunes library file
    match matches.value_of("library") {
        Some(library_file) => {
            println!("Processing from library: {:?}", library_file);
            process_library(library_file);
            return;
        }
        None => {} // some other arg must match
    }

    // otherwise, we may have been given a directory to process
    match matches.value_of("directory") {
        Some(directory) => {
            // process a library from a directory.
            return;
        }
        None => {} // some other arg must match
    }

    match matches.is_present("stream") {
        true => {
            //do stuff from stdin
        }
        false => {}
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    dispatch(matches);

    // let library_file = matches.value_of("library").unwrap();

    let profile = Profile::from_spans(flame::spans());
    profile.print();

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
