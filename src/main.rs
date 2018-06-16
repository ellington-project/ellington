extern crate plist;

mod track;

use plist::Plist;

use std::fs::File;

use track::Track;

fn extract_track(trackpl: &plist::Plist) -> Option<Track> {
    // assert the track plist is a dictionary
    let trackinfo = trackpl.as_dictionary()?;

    // build a track with information extracted from the dict
    // bail out (and return None) if we fail to get any of: 
    // - track id
    // - name
    // - location
    // fill the BPM with "none" if no bpm found
    Some(Track {
        itunes_id: trackinfo.get("Track ID")?.as_integer()?,
        bpm: trackinfo.get("BPM").and_then(|b| b.as_integer()),
        name: trackinfo.get("Name")?.as_string()?.to_string(),
        location: trackinfo
            .get("Location")?
            .as_string()?.to_string(),
    })
}

fn read_plist(filename: &str) -> () {
    let file = File::open(filename).unwrap();

    let plist = Plist::read(file).unwrap();

    // get the tracks from the PList:
    let tracks = plist.as_dictionary().unwrap().get("Tracks").unwrap();

    println!(
        "Found {} tracks in the tracklist",
        tracks.as_dictionary().unwrap().len()
    );

    let tracks = tracks
        .as_dictionary()
        .unwrap()
        .values()
        .map(extract_track);

    for t in tracks {
        match t {
            Some(track) => println!("Track: {}", track),
            None => println!("Got bad track.")
        }
    }

}

fn main() {
    read_plist("res/partialLibrary.xml");
    // read_plist("/Users/adam/Music/iTunes/iTunes Music Library.xml");
    // println!("Hello, world!");
}
