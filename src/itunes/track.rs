use plist::Plist;
use std::fmt;

#[derive(Debug)]
pub struct Track {
    pub itunes_id: i64,
    pub bpm: Option<i64>, // we might not have a bpm value
    pub name: String,
    pub location: String,
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
    pub fn new(plist: &Plist) -> Option<Track> {
        // assert the track plist is a dictionary
        let trackinfo = plist.as_dictionary()?;

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
            location: trackinfo.get("Location")?.as_string()?.to_string(),
        })
    }
}
