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

        flame::start("Plist::read");
        let plist = Plist::read(file).ok()?;
        flame::end("Plist::read");

        // get the tracks from the PList:
        flame::start("plist.as_dictionary()?.get");
        let tracks = plist.as_dictionary()?.get("Tracks")?;
        flame::end("plist.as_dictionary()?.get");

        // note, flat_map will (I assume?) discard failed tracks
        flame::start("tracks.as_dictionary()");
        let tracks_d = tracks.as_dictionary().unwrap();
        flame::end("tracks.as_dictionary()");

        flame::start("tracks_d.values()");
        let tracks_v = tracks_d.values();
        flame::end("tracks_d.values()");

        flame::start("tracks_v.flat_map()");
        let tracks_new = tracks_v.flat_map(Track::new).collect();
        flame::end("tracks_v.flat_map()");

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
