use std::fs::File;

use plist::Plist;

use itunes::track::Track;

#[derive(Debug)]
pub struct Library {
    pub tracks: Vec<Track>,
}

impl Library {
    #[flame]
    pub fn from_filename(filename: &str) -> Option<Library> {
        let file = File::open(filename).ok()?;

        let plist = Plist::read(file).ok()?;

        // get the tracks from the PList:
        let tracks = plist.as_dictionary()?.get("Tracks")?;

        println!(
            "Found {} tracks in the tracklist",
            tracks.as_dictionary().unwrap().len()
        );

        // note, flat_map will (I assume?) discard failed tracks
        let tracks = tracks
            .as_dictionary()
            .unwrap()
            .values()
            .flat_map(Track::new)
            .collect();

        Some(Library { tracks: tracks })
    }
}
