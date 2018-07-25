pub mod ellingtondata;
pub mod filemetadata;
pub mod trackmetadata;

use library::ellingtondata::*;
use library::filemetadata::FileMetadata;
use library::trackmetadata::taglib::*;
use library::trackmetadata::*;
use pipelines::Pipeline;

use percent_encoding;
use plist::Plist;
use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

use std::path::PathBuf;
use url::Url;
use walkdir::WalkDir;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub location: PathBuf,
    pub filedata: FileMetadata,
    pub metadata: Option<TrackMetadata>,
    pub eldata: EllingtonData,
}

impl Entry {
    pub fn from_file(path: PathBuf) -> Entry {
        // try to read some metadata from the track
        let filedata = FileMetadata::from_path(&path);
        // TODO: Implement different readers here!
        let metadata = TrackMetadata::from_file(&path, &filedata);
        let eldata = match &metadata {
            Some(m) => m.as_ellington_metadata(),
            None => EllingtonData::empty(),
        };
        Entry {
            location: path,
            filedata: filedata,
            metadata: metadata,
            eldata: eldata,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub tracks: Vec<Entry>,
}

impl Library {
    /*
        Read a library from an itunes xml/plist file
     */
    // #[flame]
    pub fn from_itunes_xml(filename: &str) -> Option<Library> {
        let file = File::open(filename).ok()?;

        let plist = Plist::read(file).ok()?;

        fn url_to_path(location: &String) -> PathBuf {
            let parsedurl = Url::parse(location).unwrap();

            let path_str = parsedurl.path();
            let path_bytes: Vec<u8> = path_str.bytes().collect();
            // decode it
            let decoded = percent_encoding::percent_decode(&path_bytes[..])
                .decode_utf8()
                .unwrap()
                .into_owned();
            PathBuf::from(decoded)
        }

        // get the tracks from the PList:
        let mut entries = 0;
        let tracks: Vec<Entry> = plist
            .as_dictionary()?
            .get("Tracks")?
            .as_dictionary()?
            .values()
            .flat_map(|track_plist: &Plist| -> Option<Entry> {
                // assert the track plist is a dictionary
                let trackinfo = track_plist.as_dictionary()?;

                // extract the location from the dictionary.
                let location = url_to_path(&trackinfo.get("Location")?.as_string()?.to_string());

                entries += 1;

                Some(Entry::from_file(location))
            })
            .collect();

        info!(
            "Successfully read {} tracks from the itunes library, out of {} itunes entries",
            tracks.len(),
            entries
        );

        Some(Library { tracks: tracks })
    }

    /*
        Read a library as a list of audio files, with one
        audio file path per line
     */
    // #[flame]
    pub fn from_stdin() -> Option<Library> {
        // each line in stdin is assumed to be a path to a track name
        let stdin = io::stdin();
        let mut lines = 0;
        let tracks: Vec<Entry> = stdin
            .lock()
            .lines()
            .map(|l| {
                info!("Got line: {:?}", l);
                lines += 1;
                l
            })
            .filter_map(|l| l.ok())
            .map(|line| Entry::from_file(PathBuf::from(line)))
            .collect();
        info!(
            "Successfully read {} tracks from stdin, out of {} lines",
            tracks.len(),
            lines
        );
        Some(Library { tracks: tracks })
    }

    /*
        Read a library from a directory, recursively exploring the 
        file hierarchy, and finding audio files.
     */
    // #[flame]
    pub fn from_directory_rec(path: &PathBuf) -> Option<Library> {
        let mut entries = 0;
        let mut io_errors = 0;
        let mut io_successes = 0;
        let mut bad_files: BTreeSet<PathBuf> = BTreeSet::new();
        let mut audio_files = 0;
        let tracks: Vec<Entry> = WalkDir::new(path)
            .max_open(1)
            .contents_first(true)
            .into_iter()
            .map(|e| {
                info!("Got entry: {:?}", e);
                match e {
                    Ok(ref e) => {
                        bad_files.remove(&e.path().to_path_buf());
                        io_successes += 1
                    }
                    Err(ref e) => {
                        bad_files.insert(e.path().unwrap().to_path_buf());
                        io_errors += 1;
                    }
                }
                entries += 1;
                e
            })
            .filter_map(|e| e.ok())
            .filter_map(|e| FileMetadata::seq_audio_file(e.clone(), &e.path()))
            .map(|f| {
                info!("Got audio file: {:?}", f);
                audio_files += 1;
                f
            })
            .map(|f| Entry::from_file(f.path().to_path_buf()))
            .collect();

        info!(
            "Got {} IO errors from too many open files, and {} successfully opened files, with permanently failed paths: \n{:#?}",
            io_errors, io_successes, bad_files
        );

        info!(
            "Successfully read {} tracks from directory {:?}, with {} entries, and {} audio files",
            tracks.len(),
            path,
            entries,
            audio_files
        );

        Some(Library { tracks: tracks })
    }

    /* 
        Read a library from an ellington library file, with name 
        "<library>.el"
    */

    pub fn from_file(path: &PathBuf) -> Option<Library> {
        info!("Reading library from {:?}", path);
        let json = match fs::read_to_string(path) {
            Ok(j) => Some(j),
            Err(e) => {
                error!(
                    "Error reading ellington library from file {:?}, got io error {:?}",
                    path, e
                );
                None
            }
        }?;

        match serde_json::from_str::<Library>(&json) {
            Ok(l) => Some(l),
            Err(e) => {
                error!(
                    "Failed to parse library file {:?}! Serde error {:?}",
                    path, e
                );
                None
            }
        }
    }

    /* 
        Write a library to a file
     */
    pub fn write_to_file(self: &Self, path: &PathBuf) -> Option<()> {
        let json: String = serde_json::to_string_pretty(self).expect("Couldn't serialize config");
        match fs::write(path, json) {
            Ok(()) => Some(()),
            Err(e) => {
                error!(
                    "Error writing ellington library file to {:?}, got io error {:?}",
                    path, e
                );
                None
            }
        }
    }

    pub fn run_pipeline<P: Pipeline>(self: &mut Self) -> () {
        info!("Running pipeline over ellington library.");
        info!("Using pipeline: {:?}", P::NAME);
        // iterate over our tracks, and run the pipeline
        let mut ix = 0;
        let lx = self.tracks.len();
        for entry in &mut self.tracks {
            info!(
                "Running pipeline {:?} on track {:?}/{:?}:\n\t {:?}",
                P::NAME,
                ix,
                lx,
                entry.location
            );
            ix += 1;
            // get the pipeline result.
            match P::run(&entry.location) {
                Some(calculated_bpm) => {
                    match &entry.metadata {
                        Some(m) => match m.bpm {
                            Some(b) => {
                                info!("Caculated bpm: {:?}, expected: {:?}", calculated_bpm, b)
                            }
                            _ => info!("Caculated bpm: {:?}", calculated_bpm),
                        },
                        _ => info!("Caculated bpm: {:?}", calculated_bpm),
                    }
                    entry
                        .eldata
                        .algs
                        .insert(P::NAME.to_string(), calculated_bpm);
                }
                None => {
                    error!("Failed to calculate bpm for entry: {:?}", entry);
                }
            }
        }
    }

    pub fn write_metadata_to_audio_files(self: &Self) -> () {
        for entry in &self.tracks {
            match GenericTaglibAudioFile::write_ellington_data(
                &PathBuf::from(entry.location.clone()),
                &entry.eldata,
            ) {
                Some(()) => info!("Successfully wrote metadata to file {:?}", entry.location),
                None => error!("Failed to write metadata to file {:?}", entry.location),
            };
        }
    }
}
