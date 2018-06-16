extern crate plist; 

use plist::Plist; 

use std::fs::File; 

struct Track {
    iTunesID: u32,
    bpm: u32,
    name: String, 
    location: String,  
}

fn extract_track(pldict: plist::Plist) -> Option<Track> {

    return match pldict.as_dictionary(){
        Some(dict) => {
            Some(Track {
                iTunesID: 0, 
                bpm: 0, 
                name: "name".to_string(), 
                location: "here".to_string()
            })
        }
        None => None
    }
}


fn read_plist(filename: &str) -> () {
    let file = File::open(filename).unwrap();

    let plist = Plist::read(file).unwrap(); 

    // println!("Overall Plist: {:?}", &plist);

    // get the tracks from the PList: 

    let tracks = plist.as_dictionary().unwrap().get("Tracks").unwrap();

    println!("Found {} tracks in the tracklist", tracks.as_dictionary().unwrap().len());

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
