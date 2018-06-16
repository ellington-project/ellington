extern crate plist;

mod track;

use plist::Plist;

use std::fs::File;

use track::Track;

fn extract_track(pldict: plist::Plist) -> Option<Track> {
    return pldict.as_dictionary().and_then(|dict| -> Option<Track> {
        // extract information from the dict
        let (itunes_id, trackinfo) = dict.iter().next().unwrap();
        let itunes_id: u32 = itunes_id.parse::<u32>().unwrap();

        // get more info from the trackinfo dict
        let trackinfo = trackinfo.as_dictionary()?;

        Some(Track {
            itunes_id: itunes_id,
            // not exactly the semantics we want, but good enough...
            bpm: trackinfo
                .get("BPM")
                .and_then(|bpm| bpm.as_string()?.parse::<u32>().ok()),
            // simpler operations for the rest, just string conversions
            name: trackinfo.get("Name")?.as_string().map(|s| s.to_string()),
            location: trackinfo
                .get("Location")?
                .as_string()
                .map(|s| s.to_string()),
        })
    });
}

fn read_plist(filename: &str) -> () {
    let file = File::open(filename).unwrap();

    let plist = Plist::read(file).unwrap();

    // println!("Overall Plist: {:?}", &plist);

    // get the tracks from the PList:

    let tracks = plist.as_dictionary().unwrap().get("Tracks").unwrap();

    println!(
        "Found {} tracks in the tracklist",
        tracks.as_dictionary().unwrap().len()
    );

    // println!("Tracks: {:?}", tracks);

    for (id, track) in tracks.as_dictionary().unwrap().iter() {
        println!("Found track, id: {}:", id);
        // get the dictionary for the track:
        let dict = track.as_dictionary().unwrap();
        println!("Name: {:?}", dict.get("Name").unwrap());
        println!("BPM: {:?}", dict.get("BPM").unwrap());
        // println!("\t{:?}", track);
        println!("---");
    }
}

fn main() {
    read_plist("res/partialLibrary.xml");
    // read_plist("/Users/adam/Music/iTunes/iTunes Music Library.xml");
    // println!("Hello, world!");
}
