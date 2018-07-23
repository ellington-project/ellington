use library::track::Track;
use percent_encoding;
use plist::Plist;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;
use url::Url;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Library {
    pub tracks: Vec<Box<Track>>,
}

impl Library {
    /*
        Read a library from an itunes xml/plist file
     */
    // #[flame]
    pub fn from_itunes_xml(filename: &str) -> Option<Library> {
        let file = File::open(filename).ok()?;

        let plist = Plist::read(file).ok()?;

        // get the tracks from the PList:
        let tracks = plist
            .as_dictionary()?
            .get("Tracks")?
            .as_dictionary()?
            .values()
            .flat_map(|track_plist: &Plist| -> Option<Box<Track>> {
                // assert the track plist is a dictionary
                let trackinfo = track_plist.as_dictionary()?;

                // extract the location from the dictionary.
                let location =
                    Library::url_to_path(&trackinfo.get("Location")?.as_string()?.to_string());

                // read the metadata from the file, rather than iTunes, in case there
                // are any discrepancies.
                // TODO: Control this with a flag?
                Track::from_file(&location)
            })
            .collect();

        Some(Library { tracks: tracks })
    }

    /*
        Read a library as a list of audio files, with one
        audio file path per line
     */
    // #[flame]
    #[allow(dead_code)]
    pub fn from_stdin() -> Option<Library> {
        // each line in stdin is assumed to be a path to a track name
        let stdin = io::stdin();
        let mut lines = 0;
        let tracks: Vec<Box<Track>> = stdin
            .lock()
            .lines()
            .map(|l| {
                info!("Got line: {:?}", l);
                lines += 1;
                l
            })
            .filter_map(|l| l.ok())
            .filter_map(|line| Track::from_file(&PathBuf::from(line)))
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
    #[allow(dead_code, unused_variables)]
    pub fn from_directory_rec(path: &PathBuf) -> Option<Library> {
        let mut entries = 0;
        let mut io_errors = 0;
        let mut io_successes = 0;
        let mut bad_files: BTreeSet<PathBuf> = BTreeSet::new();
        let mut audio_files = 0;
        let tracks: Vec<Box<Track>> = WalkDir::new(path)
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
            .filter_map(|e| Self::is_audio_file(e))
            .map(|f| {
                info!("Got audio file: {:?}", f);
                audio_files += 1;
                f
            })
            .filter_map(|f| Track::from_file(&f.path().to_path_buf()))
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
        General utitlity functions
     */
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

    fn is_audio_file(de: DirEntry) -> Option<DirEntry> {
        if de.file_type().is_dir() {
            None
        } else {
            let d = de.clone();
            de.path().extension().and_then(|ext| match ext.to_str() {
                Some("flac") => Some(d),
                Some("m4a") => Some(d),
                Some("m4p") => Some(d),
                Some("mp3") => Some(d),
                Some("mp4") => Some(d),
                Some("wav") => Some(d),
                Some("alac") => Some(d),
                _ => None,
            })
        }
    }
}
