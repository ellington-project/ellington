// #![feature(plugin, custom_attribute)]
// #![plugin(flamer)]

extern crate byteorder;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

// extern crate slog; 
// extern crate slog_term; 
// extern crate slog_scope; 
// extern crate flame;
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
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

extern crate regex;
#[macro_use]
extern crate lazy_static;

extern crate taglib;

mod analysers;
mod input;
mod library;
mod profiling;
mod shelltools;

use clap::ArgMatches;
// use profiling::Profile;

use library::ellingtondata::BpmInfo;
use library::ellingtondata::EllingtonData;
// use input::audiobuffer::AudioBuffer;
use input::audiostream::AudioStream;

use analysers::bpmtools::BpmTools;

use shelltools::sox::*;
use shelltools::ffmpeg::*;

use library::library::Library;

use histogram::Histogram;

use clap::App;
use std::path::PathBuf;

// use slog::*;

// #[flame]
fn percent_err(gold: f64, trial: f64) -> f64 {
    return ((gold - trial).abs() / gold) * 100.0;
}

// #[flame]
fn print_histogram(h: &Histogram, div: f64) -> () {
    info!(
        "Percentiles -- 75: {} 80: {} 85: {} 90: {} 95: {}",
        h.percentile(75.0).unwrap_or(9999999) as f64 / div,
        h.percentile(80.0).unwrap_or(9999999) as f64 / div,
        h.percentile(85.0).unwrap_or(9999999) as f64 / div,
        h.percentile(90.0).unwrap_or(9999999) as f64 / div,
        h.percentile(95.0).unwrap_or(9999999) as f64 / div,
    );
}

// #[flame]
fn process_library(library: Library) -> () {

    // Parse the library from an itunes file
    // let library = Library::from_itunes_xml(filename).unwrap();

    info!("Successfully parsed {} tracks.", library.tracks.len());
    
    let mut error_hist = Histogram::new();

    // Iterate over the tracks.
    for track in library.tracks {
        // flame::start("process_track");

        // Match the tracks that contain ellington data
        match track.ellington_data() {
            // If we have ellington data
            Some(ed) => {
                info!("Track: {}", track);
                info!("Bpm: {:?}", track.bpm());
                info!("Comment: {:#?}", track.comments());
                info!("Ed: {:#?}", ed);

                let mut call = FfmpegCommand::default(&track.location());
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

                info!("Calculated ffmpeg bpm: {}", cbpm);

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

                info!("Calculated sox bpm: {}", cbpm);

                // build some ellington data 
                // let new_data = EllingtonData { 
                //     algs: Some (
                //         vec![BpmInfo{
                //             bpm: cbpm as i64, 
                //             alg: "naive".to_string()
                //         }]
                //     )
                // };

                // match track.write_data(new_data) {
                //     Some(_) => info!("Successfully written data."), 
                //     None => info!("Failed to write id3 data for some reason.")
                // }
                
                info!("===== ===== ===== ===== =====\n");
            }
            _ => {
                info!("Ignore... {:?}", track.name());
            }
        }

        // flame::end("process_track");
    }
}

// #[flame]
fn dispatch(matches: ArgMatches) -> () {

    let library = match (matches.value_of("library"), matches.value_of("directory"), matches.is_present("stream")) { 
        (Some(library_file), _, _ ) => {
            info!("Processing from library: {:?}", library_file);
            Library::from_itunes_xml(library_file)
        }
        (_, Some(directory), _ ) => {
            info!("Reading from directory: {}", directory);
            Library::from_directory_rec(&PathBuf::from(directory))
        }
        (_, _, true) => {
            info!("Reading track file names from stdin.");
            None
        }
        _ => {
            error!("Should not reach here!"); 
            None
        }
    };

    info!("Got library: {:?}", library);
    // process_library(library.unwrap());
}

fn main() {
    env_logger::init();
    // get the command line arguments to the program
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    // let mut version = String::new();
    // app.write_version(&mut version);

    // start logging
    // let drain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    // let root_logger = Logger::root(slog_term::FullFormat::new(drain).build().fuse(), o!("version" => "0.1.0"));
    // let _guard = slog_scope::set_global_logger(root_logger);

    // slog_stdlog::init().unwrap();

    info!("Application started");

    dispatch(matches);

    // let library_file = matches.value_of("library").unwrap();

    // let profile = Profile::from_spans(flame::spans());
    // profile.print();

    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
