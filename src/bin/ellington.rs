/*
    ellington - the ellington tool for processing and bpming audio libraries
*/
use std::path::PathBuf;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate nom;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;
use std::path::Path;

// extern crate commandspec;
// use commandspec::*;

extern crate ellington;

use ellington::library::ellingtondata::EllingtonData;
use ellington::library::trackmetadata::*;
use ellington::library::Entry;
use ellington::library::Library;

use ellington::estimators::BellsonTempoEstimator;
use ellington::estimators::FfmpegNaiveTempoEstimator;
use ellington::estimators::TempoEstimator;

use ellington::types::*;

// fn check_callable(program: &'static str) -> Option<()> {
//     //TODO: this needs to be written to capture the various output streams, as it pollutes ellington's output otherwise
//     match execute!(r"which {program}", program = program) {
//         Err(_) => {
//             println!("Cannot find program '{}' - please make sure it's installed before running this command", program);
//             None
//         }
//         _ => Some(()),
//     }
// }

// #[flame]
fn init(matches: &ArgMatches) -> () {
    /*
        Step one, work out what our audio source will be for tracks:
    */
    let library: Library = match matches.value_of("SOURCE").unwrap() {
        "empty" => {
            info!("Initialising empty library");
            Library::from_empty()
        }
        "stdin" => {
            info!("Reading tracks from stdin");
            Library::from_stdin()
        }
        "directory" => matches.value_of("directory").and_then(|directory| {
            info!("Reading from directory: {}", directory);
            Library::from_directory_rec(&PathBuf::from(directory))
        }),
        "itunes" => matches.value_of("itunes").and_then(|library_file| {
            info!("Processing from itunes library: {:?}", library_file);
            Library::from_itunes_xml(library_file)
        }),
        _ => None,
    }.unwrap();

    /*
        Step two, work out where to write the cache:
    */
    let library_file: &str = matches
        .value_of("LIBRARY")
        .and_then(|l| {
            info!("Writing library to: {:?}", l);
            Some(l)
        }).unwrap();

    // Create directories for the library file
    match Path::new(library_file)
        .parent()
        .and_then(|p| {
            println!("Got path {:?}", p);
            Some(p)
        }).or_else(|| {
            println!("Could not get parent directory!");
            None
            // TODO: This code is quite useful, but brittle! It needs to be rethought...
            // }).and_then(|parent| match fs::canonicalize(parent) {
            //     Ok(path) => Some(path),
            //     Err(e) => {
            //         println!("Failed to canonicalise path with error: {}", e);
            //         None
            //     }
        }) {
        Some(parent_path) => std::fs::create_dir_all(parent_path).unwrap(),
        None => {
            panic!("Could not get directory in which to create ellington library, found error.")
        }
    };

    // Write the computed library to the file
    library.write_to_file(&PathBuf::from(library_file));
}

fn query_estimator(
    algorithm: AlgorithmE,
    caches: &Vec<EllingtonData>,
    force: bool,
    f: impl Fn() -> Option<i64>,
) -> BpmE {
    if force {
        return BpmE::from_option(f());
    }
    for cache in caches {
        match cache.algs.get(&algorithm) {
            Some(tmpo) => return tmpo.clone(),
            _ => {}
        }
    }
    BpmE::from_option(f())
}

fn query(matches: &ArgMatches) -> () {
    /*  A query runs in the following fashion: 
        1 - Get the name of the file that we want to query information on. 
        2 - Load the cache, and read metadata from the audio file and library
        3 - Select the list of estimators that we want to query. 
        4 - For each estimator:
            > Check what our estimator preferences are. 
                . If eager, we might as well just run them, and ignore the cache completely. 
                . If lazy, we can run estimators after reading from the cache, if we have no values. 
            > Append the result to the list of estimators 
        5 - Write to the library if --pure is not specified
        6 - Print the output: 
            > If json, print the output in json
            > If readable, print the output human readably
            > else, serialise (checking for --minimal), and print the result.
    */

    /*
        1. Get the name of the audio file, and look up the ellington library that we've been passed. 
    */
    let audio_file: &str = matches
        .value_of("AUDIOFILE")
        .and_then(|ap| {
            info!("Processing audio file at {:?}", ap);
            Some(ap)
        }).unwrap();

    let library_file: &str = matches
        .value_of("LIBRARY")
        .and_then(|l| {
            info!("Writing library to: {:?}", l);
            Some(l)
        }).unwrap();

    /* 
        2. Load data from the cache, and metadata from the audio file
    */
    // Load the library "cache"
    let library = Library::read_from_file(&PathBuf::from(library_file))
        .and_then(|l| {
            info!("Read library successfully!");
            Some(l)
        }).or_else(|| {
            error!("Failed to read ellington library!");
            None
        });

    // Get the entry of the audio file
    let library_entry: Option<Entry> = library.clone().and_then(|l| {
        l.lookup(&PathBuf::from(audio_file))
            .and_then(|e| {
                info!("Found entry in library!");
                Some(e.clone())
            }).or_else(|| {
                error!("Could not find track in library!");
                None
            })
    });

    // Get the ellington data from that entry
    let library_eldata: EllingtonData = library_entry
        .and_then(|e| Some(e.eldata.clone()))
        .unwrap_or(EllingtonData::empty());

    // Load the track data from the audio file
    let track_metadata: Option<TrackMetadata> = TrackMetadata::from_file(Path::new(audio_file));

    // get data from the comments
    let comment_eldata: EllingtonData = track_metadata
        .as_ref()
        .and_then(|tm| Some(tm.comment_metadata()))
        .unwrap_or(EllingtonData::empty());

    // get data from the title
    let title_eldata: EllingtonData = track_metadata
        .as_ref()
        .and_then(|tm| Some(tm.title_metadata()))
        .unwrap_or(EllingtonData::empty());

    info!("Library metadata: {:?}", library_eldata);
    info!("Title metadata: {:?}", title_eldata);
    info!("Comment metadata: {:?}", comment_eldata);

    /*
        3. Select the list of estimators that we want to query. 
    */
    let estimator: &str = matches
        .value_of("estimators")
        .and_then(|e| {
            info!("Running estimator: {:?}", e);
            Some(e)
        }).unwrap();

    // Check to see if we need to forcibly run them.
    let force = matches.occurrences_of("force") > 0;

    /*
        4. Start iterating over estimators. 
    */

    // Create the ellington data for the estimators. This is where we will store the results of running our estimators.
    // Initialise it based on the "prefer" argument on the command line.
    let caches = match matches.value_of("prefer_source").unwrap() {
        "library" => vec![library_eldata, title_eldata, comment_eldata],
        "title" => vec![title_eldata, library_eldata, comment_eldata],
        "comments" => vec![comment_eldata, library_eldata, title_eldata],
        "userdata" => vec![library_eldata],
        _ => panic!("We should always get a priority, this should not happen!"),
    };

    let mut ed = EllingtonData::empty();

    // Start with the "actual" value
    if estimator == AlgorithmE::Actual.print() || estimator == "all" {
        let tempo = query_estimator(AlgorithmE::Actual, &caches, force, || {
            TrackMetadata::from_file(Path::new(audio_file)).and_then(|tmd| tmd.bpm)
        });
        ed.algs.insert(AlgorithmE::Actual, tempo);
    }

    // Run bellson, and try to add the result.
    if estimator == AlgorithmE::Bellson.print() || estimator == "all" {
        let tempo = query_estimator(AlgorithmE::Actual, &caches, force, || {
            BellsonTempoEstimator::run(&PathBuf::from(audio_file))
        });
        ed.algs.insert(BellsonTempoEstimator::ALGORITHM, tempo);
    }

    // Run the naive estimator
    if estimator == AlgorithmE::Naive.print() || estimator == "all" {
        let tempo = query_estimator(AlgorithmE::Naive, &caches, force, || {
            FfmpegNaiveTempoEstimator::run(&PathBuf::from(audio_file))
        });
        ed.algs.insert(FfmpegNaiveTempoEstimator::ALGORITHM, tempo);
    }

    /*
        5 - Write to the library if --pure is not specified
    */
    if !matches.occurrences_of("force") > 0 {
        // Check that we have a library in the first place!
        // This unwrap should be guaranteed to be safe!
        let mut new_library = library
            .and_then(|lib| Some(lib.clone()))
            .or_else(|| Library::from_empty())
            .unwrap();

        new_library.update(&PathBuf::from(audio_file), ed.clone());

        // Write the computed library to the file
        new_library.write_to_file(&PathBuf::from(library_file));
    }

    /*
        6 - Print the output: 
            > If json, print the output in json
            > If readable, print the output human readably
            > else, serialise (checking for --minimal), and print the result.
    */
    // check to see what kind of output the user has requested.
    let minimal = matches.occurrences_of("minimal") > 0;

    // Check the comment that we've got, and try to either
    //  a) update it, or
    //  b) create a new one.
    match matches.value_of("comment") {
        Some(c) => {
            match ed.update_data(&String::from(c), true, minimal) {
                Ok(new_comment) => {
                    info!("Got new comment: {:?}", new_comment);
                    println!("{}", new_comment);
                }
                f => {
                    info!("Updating procedure failed for reason: {:?}", f);
                }
            };
        }
        None => match ed.format(minimal) {
            Ok(new_comment) => {
                info!("Got new comment: {:?}", new_comment);
                println!("{}", new_comment);
            }
            f => {
                info!("Updating procedure failed for reason: {:?}", f);
            }
        },
    };
}

fn main() {
    env_logger::init();
    // get the command line arguments to the program
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let mut appm = app.clone();
    let matches = app.get_matches();
    let subcommands = matches.subcommand();

    info!("Application started");

    match subcommands {
        ("init", Some(sub)) => init(sub),
        ("query", Some(sub)) => query(sub),
        _ => appm.print_help().unwrap(),
    };
}
