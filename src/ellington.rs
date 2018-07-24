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

extern crate libellington as le;

use le::library::Library;
use le::pipelines::FfmpegNaivePipeline;

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

    let mut library = Library::from_file(&PathBuf::from(library_file)).unwrap();

    library.run_pipeline::<FfmpegNaivePipeline>();

    library.write_to_file(&PathBuf::from(library_file));
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
        _ => error!("No subcommand given!"),
    }
}
