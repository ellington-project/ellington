use library::ellingtondata::*;
use std::collections::BTreeMap;
use std::path::Path;

use talamel::*;

// a structure storing metadata about some track, in a format agnostic manner
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackMetadata {
    pub name: String,                  // we must always have a track name
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

impl TrackMetadata {
    #[flame("TrackMetadata")]
    pub fn comment_metadata(self: &Self) -> EllingtonData {
        // initialise our array
        let mut algs: BTreeMap<Algorithm, Bpm> = BTreeMap::new();
        // match the comments, and iterate over them, appending them to "data"
        match &self.comments {
            Some(v) => {
                for c in v {
                    // parse the comment into some ellington data
                    match EllingtonData::parse(&c) {
                        Some(mut ed) => {
                            info!("Found ellington metadata: {:?}", ed);
                            // There _will_ be a bug here if we accidentally insert "na" after a good value.
                            // Move this into the ellington data + method?
                            algs.append(&mut ed.algs);
                        }
                        None => info!("No ellington data found in comment: {:?}", c),
                    };
                }
            }
            None => info!("Got no comments from metadata."),
        };
        EllingtonData { algs: algs }
    }

    #[flame("TrackMetadata")]
    pub fn title_metadata(self: &Self) -> EllingtonData {
        // initialise our array
        let mut algs: BTreeMap<Algorithm, Bpm> = BTreeMap::new();
        // check the track name (title) to see if it has metadata
        match EllingtonData::parse(&self.name) {
            Some(mut ed) => {
                info!("Found ellington metadata: {:?}", ed);
                algs.append(&mut ed.algs);
            }
            None => info!("No ellington data found in title: {:?}", self.name),
        }
        EllingtonData { algs: algs }
    }

    #[flame("TrackMetadata")]
    pub fn from_file(location: &Path) -> Option<TrackMetadata> {
        info!("Reading track metadata from: {:?}", location);
        let tf = TalamelFile::new(location).ok()?;

        let name = tf.title().ok()?;
        let bpm = tf.bpm().map(|b| b as i64);
        let comments = tf.comments().ok();

        Some(TrackMetadata {
            name: name,
            bpm: bpm,
            comments: comments,
        })
    }
}
