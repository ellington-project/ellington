use std::path::Path;
use library::track::Track;
use percent_encoding;
use std::fs::File;
use std::path::PathBuf;
use url::Url;
use walkdir::WalkDir;
use walkdir::DirEntry;

use plist::Plist;

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

                // build a track with information extracted from the dict
                // bail out (and return None) if we fail to get any of:
                // - track id
                // - name
                // - location
                // fill the BPM with "none" if no bpm found
                // let bpm = trackinfo.get("BPM").and_then(|b| b.as_integer());
                // let comment : Option<String> = trackinfo
                //     .get("Comments")
                //     .and_then(|c| c.as_string())
                //     .and_then(|s| Some(s.to_string()));
                // let name = trackinfo.get("Name")?.as_string()?.to_string();
                // let audioformat = AudioFormat::from_path(&location);
                // let ellingtondata = comment.clone().and_then(|s| EllingtonData::parse_data(&s));
                // Some(Box::new(Track {
                //     bpm: bpm,
                //     comment: comment,
                //     name: name,
                //     location: location,
                //     audioformat: audioformat,
                //     metadata: ellingtondata
                // }))
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
        unimplemented!()
    }

    /*
        Read a library from a directory, recursively exploring the 
        file hierarchy, and finding audio files.
     */
    // #[flame]
    #[allow(dead_code, unused_variables)]
    pub fn from_directory_rec(path: &PathBuf) -> Option<Library> {
        let mut files = 0;
        let mut directories = 0;
        let mut total_tracks = 0; 
        let mut failed_tracks = 0;
        let tracks : Vec<Option<PathBuf>> = WalkDir::new(path).contents_first(true).into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| Self::is_audio_file(e))
            .map(|f| {
                info!("Reading from path {:?}", f.path());
                match f.file_type().is_dir() { 
                    true => {
                        directories += 1;
                        None
                    }, 
                    false => {
                        files += 1;
                        Some(f.path().to_path_buf())
                    }
                }
            }).collect();
        // let tracks: Vec<Box<Track>> = WalkDir::new(path).contents_first(true)
        //     .into_iter()
        
        //     .filter_map(|e| e.ok())
        //     .filter_map(|f| {
        //         info!("Reading track from path: ${:#?}", f);
        //         total_tracks += 1; 
        //         match Track::from_file(&f.path().to_path_buf()) {
        //             None => {
        //                 error!("Could not read tag from ${:#?}", f);
        //                 failed_tracks += 1;
        //                 None
        //             }, 
        //             t => t
        //         }
        //     })
        //     .collect();

        error!("Failed to read {:?} tracks out of {:?}, with {:?} files, {:?} directories", failed_tracks, total_tracks, files, directories);
        None
        // match tracks.len() {
        //     0 => {
        //         info!("No tracks found in directory!");
        //         None
        //     }
        //     a => {
        //         info!("Found {:?} tracks", a);
        //         Some(Library { tracks: tracks })
        //     }
        // }
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
        }else{
            let d = de.clone();
            de.path().extension().and_then(|ext| 
                match ext.to_str() { 
                    Some("flac") => Some(d),
                    Some("m4a") => Some(d),
                    Some("m4p") => Some(d),
                    Some("mp3") => Some(d),
                    Some("mp4") => Some(d),
                    Some("wav") => Some(d),
                    Some("alac") => Some(d), 
                    _ => None
                }
            )
        }
    }
}
