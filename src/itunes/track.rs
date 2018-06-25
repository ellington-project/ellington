use std::str::from_utf8;
use plist::Plist;
use shelltools::generic::EscapedFilename;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use url::Url;
use percent_encoding;

#[derive(Debug)]
pub struct Track {
    pub itunes_id: i64,
    pub bpm: Option<i64>, // we might not have a bpm value
    pub comment: Option<String>, // or a comment!
    pub name: String,
    pub location: PathBuf,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // All this formatting might be slow...
        let bpm_s = match self.bpm {
            Some(bpm) => format!("{:?}", bpm),
            None => " - ".to_string(),
        };
        write!(
            f,
            "(id: {} // bpm: {} // name: {} // loc : [...])",
            self.itunes_id, bpm_s, self.name
        )
    }
}

impl Track {
    // TODO: Better error handling! 
    fn url_to_path(location: &String) -> PathBuf {
        let parsedurl = Url::parse(location).unwrap();
        let path_str = parsedurl.path();
        let path_bytes : Vec<u8> = path_str.bytes().collect();
        // decode it 
        let decoded = percent_encoding::percent_decode(&path_bytes[..]).decode_utf8().unwrap().into_owned();
        println!("Got path: {:?}", decoded);
        PathBuf::from(decoded)
    }

    pub fn new(plist: &Plist) -> Option<Track> {
        // assert the track plist is a dictionary
        let trackinfo = plist.as_dictionary()?;
      
        // build a track with information extracted from the dict
        // bail out (and return None) if we fail to get any of:
        // - track id
        // - name
        // - location
        // fill the BPM with "none" if no bpm found
        let itunes_id = trackinfo.get("Track ID")?.as_integer()?;
        let bpm = trackinfo.get("BPM").and_then(|b| b.as_integer()); 
        let comment = trackinfo.get("Comments").and_then(|c| c.as_string()).and_then(|s| Some(s.to_string()));
        let name = trackinfo.get("Name")?.as_string()?.to_string();
        let location = Track::url_to_path(&trackinfo.get("Location")?.as_string()?.to_string());
        Some(Track {
            itunes_id: itunes_id,
            bpm: bpm, 
            comment: comment,
            name: name,
            location: location,
        })
    }
}
