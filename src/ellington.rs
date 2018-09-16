/*
    ellington - the ellington tool for processing and bpming audio libraries
*/

use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

extern crate commandspec;
use commandspec::*;

extern crate libellington as le;

use le::library::Library;
use le::estimators::FfmpegNaiveTempoEstimator;

fn check_callable(program: &'static str) -> Option<()> {
    match execute!(r"which {program}", program=program)  { 
        Err(_) => {
            println!("Cannot find program '{}' - please make sure it's installed before running this command", program);
            None
        },
        _ => Some(())
    }
}

// #[flame]
fn initalise_library(matches: &ArgMatches) -> () {
    // get the path we wish to write the library file to
    let library_file: &str = match matches.value_of("LIBRARY") {
        Some(l) => {
            info!("Writing library to: {:?}", l);
            l
        }
        None => {
            panic!("Got no library file, this should not happen!");
        }
    };

    let library: Library = match (matches.value_of("itunes"), matches.value_of("directory")) {
        (Some(library_file), _) => {
            info!("Processing from itunes library: {:?}", library_file);
            Library::from_itunes_xml(library_file)
        }
        (_, Some(directory)) => {
            info!("Reading from directory: {}", directory);
            Library::from_directory_rec(&PathBuf::from(directory))
        }
        _ => {
            info!("Reading tracks from stdin");
            Library::from_stdin()
        }
    }.unwrap();

    library.write_to_file(&PathBuf::from(library_file));
}

fn bpm_library(matches: &ArgMatches) -> () {
    let library_file: &str = match matches.value_of("LIBRARY") {
        Some(l) => {
            info!("Writing library to: {:?}", l);
            l
        }
        None => {
            panic!("Got no library file, this should not happen!");
        }
    };

    check_callable("ffmpeg").unwrap();

    let mut library = Library::read_from_file(&PathBuf::from(library_file)).unwrap();

    library.run_pipeline::<FfmpegNaiveTempoEstimator>();

    library.write_to_file(&PathBuf::from(library_file));
}

fn write_library(matches: &ArgMatches) -> () {
    let library_file: &str = match matches.value_of("LIBRARY") {
        Some(l) => {
            info!("Reading library from: {:?}", l);
            l
        }
        None => {
            panic!("Got no library file, this should not happen!");
        }
    };

    let append = matches.is_present("append");

    let library = Library::read_from_file(&PathBuf::from(library_file)).unwrap();

    library.write_metadata_to_audio_files(append);
}

fn clear_audio_files(matches: &ArgMatches) -> () {
    let library_file: &str = match matches.value_of("LIBRARY") {
        Some(l) => {
            info!("Reading library from: {:?}", l);
            l
        }
        None => {
            panic!("Got no library file, this should not happen!");
        }
    };

    check_callable("id3v2").unwrap();
    check_callable("mp4info").unwrap();
    check_callable("mp4tags").unwrap();

    let library = Library::read_from_file(&PathBuf::from(library_file)).unwrap();

    library.clear_data_from_audio_files();
}

fn main() {
    env_logger::init();
    // get the command line arguments to the program
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    let subcommands = matches.subcommand();

    info!("Application started");

    match subcommands {
        ("init", Some(sub)) => initalise_library(sub),
        ("bpm", Some(sub)) => bpm_library(sub),
        ("write", Some(sub)) => write_library(sub),
        ("clear", Some(sub)) => clear_audio_files(sub),
        _ => println!("No command given to ellington - please specify one of init/bpm/write/clear"),
    }
}
