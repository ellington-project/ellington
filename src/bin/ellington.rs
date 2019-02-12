/*
    ellington - the ellington tool for processing and bpming audio libraries
*/
use std::fs;
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
extern crate flame;
#[macro_use]
extern crate flamer;

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

#[flame]
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
    }
    .unwrap();

    /*
        Step two, work out where to write the cache:
    */
    let library_file: &str = matches
        .value_of("LIBRARY")
        .and_then(|l| {
            info!("Writing library to: {:?}", l);
            Some(l)
        })
        .unwrap();

    // Create directories for the library file
    match Path::new(library_file)
        .parent()
        .and_then(|p| {
            println!("Got path {:?}", p);
            Some(p)
        })
        .or_else(|| {
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

// hacky shit
#[flame]
fn dump(matches: &ArgMatches) -> () {
    let library_file: &str = matches
        .value_of("LIBRARY")
        .and_then(|l| {
            info!("Reading from library: {:?}", l);
            Some(l)
        })
        .unwrap();

    // Try to load it from a file:
    let lib: Library = Library::read_from_file(&PathBuf::from(library_file))
        .and_then(|l| {
            info!("Read library successfully!");
            Some(l)
        })
        .or_else(|| {
            error!("Failed to read ellington library!");
            None
        })
        .unwrap();

    let data: char = match matches.value_of("value").unwrap() {
        "location" => 'l',
        "title" => 't',
        _ => panic!("We should always get a value, this should not happen!"),
    };

    for track in lib.tracks {
        if data == 'l' {
            println!("{}", track.location.to_str().unwrap());
        } else if data == 't' {
            println!("{}", track.metadata.unwrap().name);
        }
    }
}

#[flame]
fn query_estimator(
    algorithm: AlgorithmE,
    caches: &Vec<EllingtonData>,
    force: bool,
    never: bool,
    f: impl Fn() -> Option<i64>,
) -> BpmE {
    info!("Querying estimator '{}'", algorithm.print());

    // Force will never conflict with never, so we don't need to check it as well
    if force {
        return BpmE::from_option(f());
    }
    // Run through the caches to search for the algorithm
    for cache in caches {
        match cache.algs.get(&algorithm) {
            Some(BpmE::NA) => info!("NA found in cache, ignoring"),
            Some(tmpo) => return tmpo.clone(),
            _ => info!("Algorithm not in cache"),
        }
    }

    // If it's not found, run the estimator, so long as 'never' has not been specified.
    if never {
        BpmE::NA
    } else {
        BpmE::from_option(f())
    }
}

#[flame]
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
            > Check to see what kind of output the user wants:
                . If substitution, substitute the ellingtondata with new data, in the format requested
                . If report, check what format, and print it.
    */

    /*
        1. Get the name of the audio file, and look up the ellington library that we've been passed.

        Do some hacky shit to get the canonical path of the audio file, so that when we pass it to stuff it's correct.
    */

    let audio_file: &str = matches
        .value_of("AUDIOFILE")
        .and_then(|ap| {
            info!("Processing audio file at {:?}", ap);
            Some(ap)
        })
        .unwrap();

    // The canonical path of the audio
    let audio_path: PathBuf = fs::canonicalize(audio_file).unwrap();

    let library_file: &str = matches
        .value_of("LIBRARY")
        .and_then(|l| {
            info!("Writing library to: {:?}", l);
            Some(l)
        })
        .unwrap();

    /*
        2. Load data from the cache, and metadata from the audio file
    */
    // Load the library "cache"
    let library = Library::read_from_file(&PathBuf::from(library_file))
        .and_then(|l| {
            info!("Read library successfully!");
            Some(l)
        })
        .or_else(|| {
            error!("Failed to read ellington library!");
            None
        });

    // Get the entry of the audio file
    let library_entry: Option<Entry> = library.clone().and_then(|l| {
        l.lookup(&PathBuf::from(audio_file))
            .and_then(|e| {
                info!("Found entry in library!");
                Some(e.clone())
            })
            .or_else(|| {
                error!("Could not find track in library!");
                None
            })
    });

    // Get the (cached) track metadata from the library
    let _library_trackdata: Option<TrackMetadata> =
        library_entry.clone().and_then(|e| e.metadata.clone());

    // Get the ellington data from that entry
    let library_eldata: EllingtonData = library_entry
        .and_then(|e| Some(e.eldata.clone()))
        .unwrap_or(EllingtonData::empty());

    // Load the track data from the audio file
    let track_metadata: Option<TrackMetadata> = TrackMetadata::from_file(audio_path.as_path());

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
            info!("Given estimator: {:?}", e);
            Some(e)
        })
        .unwrap();

    // Check to see if we need to forcibly run them.
    let force: bool = matches.occurrences_of("force") > 0;

    // Or if we're not allowed to run them!
    let never: bool = matches.occurrences_of("never") > 0;

    /*
        4. Start iterating over estimators.
    */

    // Create the ellington data for the estimators. This is where we will store the results of running our estimators.
    // Initialise it based on the "prefer" argument on the command line.
    let caches: Vec<EllingtonData> = match matches.value_of("prefer_source").unwrap() {
        "library" => vec![library_eldata, title_eldata, comment_eldata],
        "title" => vec![title_eldata, library_eldata, comment_eldata],
        "comments" => vec![comment_eldata, library_eldata, title_eldata],
        "userdata" => vec![library_eldata],
        _ => panic!("We should always get a priority, this should not happen!"),
    };

    let mut ed = EllingtonData::empty();

    // Start with the "actual" value
    // TODO: Should be able to read this from the library as well!
    if estimator == AlgorithmE::Actual.print() || estimator == "all" {
        info!("Running estimator {}", AlgorithmE::Actual.print());
        let tempo = query_estimator(AlgorithmE::Actual, &caches, force, never, || {
            TrackMetadata::from_file(audio_path.as_path()).and_then(|tmd| tmd.bpm)
        });
        ed.algs.insert(AlgorithmE::Actual, tempo);
    }

    // Run bellson, and try to add the result.
    if estimator == AlgorithmE::Bellson.print() || estimator == "all" {
        info!("Running estimator {}", AlgorithmE::Bellson.print());
        let tempo = query_estimator(AlgorithmE::Bellson, &caches, force, never, || {
            BellsonTempoEstimator::run(&audio_path)
        });
        info!("Got result {:?} from estimator.", tempo);
        ed.algs.insert(BellsonTempoEstimator::ALGORITHM, tempo);
    }

    // Run the naive estimator
    if estimator == AlgorithmE::Naive.print() || estimator == "all" {
        info!("Running estimator {}", AlgorithmE::Naive.print());
        let tempo = query_estimator(AlgorithmE::Naive, &caches, force, never, || {
            FfmpegNaiveTempoEstimator::run(&audio_path)
        });
        info!("Got result {:?} from estimator.", tempo);
        ed.algs.insert(FfmpegNaiveTempoEstimator::ALGORITHM, tempo);
    }

    /*
        5 - Write to the library if --pure is not specified
    */
    if !matches.occurrences_of("pure") > 0 {
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
            > Check to see what kind of output the user wants:
                . If substitution, substitute the ellingtondata with new data, in the format requested
                . If report, check what format, and print it.
    */
    // check to see what output behaviour the user has requested.

    match matches.value_of("output") {
        Some("update") => {
            info!("Updating metadata passed in.");
            let minimal = matches.occurrences_of("minimal") > 0;
            let modification = UpdateBehaviour::parse(matches.value_of("modification").unwrap());

            match matches.value_of("metadata") {
                Some("none") => {
                    // If none - just print the formatted output
                    println!("{}", ed.format(minimal).unwrap())
                }
                Some("title") => {
                    // If title, update the title
                    info!("Updating title data.");
                    let trmeta = track_metadata
                        .unwrap_or_else(|| panic!("No metadata found for track, failing!"));
                    match ed.update_data(&trmeta.name, modification, minimal) {
                        Ok(s) => println!("{}", s),
                        Err(e) => panic!("Could not update metadata in string! Error: {:?}", e),
                    }
                }
                Some("comments") => {
                    info!("Updating data from comment 0!");
                    let trmeta = track_metadata
                        .unwrap_or_else(|| panic!("No metadata found for track, failing!"));
                    match trmeta.comments {
                        Some(v) => match ed.update_data(&v[0], modification, minimal) {
                            Ok(s) => println!("{}", s),
                            Err(e) => panic!("Could not update metadata in string! Error: {:?}", e),
                        },
                        _ => println!("{}", ed.format(minimal).unwrap()),
                    }
                }
                Some("userdata") => {
                    // Read the userdata that we have been passed.
                    match matches.value_of("userdata") {
                        Some(u) => match ed.update_data(&String::from(u), modification, minimal) {
                            Ok(s) => println!("{}", s),
                            Err(e) => panic!("Could not update metadata in string! Error: {:?}", e),
                        },
                        _ => println!("{}", ed.format(minimal).unwrap()),
                    }
                }
                _ => panic!("Metadata not recognised or given!"),
            }
        }
        Some("report") => {
            info!("Printing data for parsing/reading.");
            match matches.value_of("format") {
                Some("json") => println!("{}", ed.format_json().unwrap()),
                Some("human") => print!("{}", ed.format_readable().unwrap()),
                _ => panic!("Format data not recognised or given!"),
            }
        }
        _ => {
            panic!("Output behaviour not recognised or given!");
        }
    };
}

#[flame]
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
        ("dump", Some(sub)) => dump(sub),
        ("query", Some(sub)) => query(sub),
        _ => {
            appm.print_help().unwrap();
            println!();
        }
    };
    {
        use std::fs::File;
        flame::dump_stdout();
        // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    }
}
