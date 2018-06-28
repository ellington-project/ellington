use flame;
use library::track::Track;
use std::fs::File;
use std::path::PathBuf;

use plist::Plist;

#[derive(Debug)]
pub struct Library {
    pub tracks: Vec<Track>,
}

impl Library {
    /*
        Read a library from an itunes xml/plist file
     */
    #[flame]
    pub fn from_itunes_xml(filename: &str) -> Option<Library> {
        let file = File::open(filename).ok()?;

        let plist = Plist::read(file).ok()?;

        // get the tracks from the PList:
        let tracks = plist.as_dictionary()?.get("Tracks")?;

        // note, flat_map will (I assume?) discard failed tracks
        let tracks_d = tracks.as_dictionary().unwrap();

        let tracks_v = tracks_d.values();

        let tracks_new = tracks_v.flat_map(Track::new).collect();

        Some(Library { tracks: tracks_new })
    }

    /*
        Read a library as a list of audio files, with one
        audio file path per line
     */
    #[flame]
    #[allow(dead_code)]
    pub fn from_stdin() -> Option<Library> {
        unimplemented!()
    }

    /*
        Read a library from a directory, recursively exploring the 
        file hierarchy, and finding audio files.
     */
    #[flame]
    #[allow(dead_code, unused_variables)]
    pub fn from_directory_rec(path: &PathBuf) -> Option<Library> {
        unimplemented!()
    }
}
