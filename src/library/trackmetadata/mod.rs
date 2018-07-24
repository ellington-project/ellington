use taglib::*;

use library::ellingtondata::*;
use std::collections::BTreeMap;
use std::path::Path;

// a structure storing metadata about some track, in a format agnostic manner
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMetadata {
    pub name: String,                  // we must always have a track name
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

impl TrackMetadata {
    pub fn as_ellington_metadata(self: &Self) -> EllingtonData {
        // initialise our array
        let mut algs: BTreeMap<Algorithm, Bpm> = BTreeMap::new();
        // match the comments, and iterate over them, appending them to "data"
        match &self.comments {
            Some(v) => {
                for c in v {
                    // parse the comment into some ellington data
                    match EllingtonData::parse_data(&c) {
                        Some(mut ed) => {
                            info!("Found ellington metadata: {:?}", ed);
                            algs.append(&mut ed.algs);
                        }
                        None => info!("No ellington data found in comment."),
                    };
                }
            }
            None => info!("Got no comments from metadata, thus no ellington data."),
        };
        EllingtonData { algs: algs }
    }
}

// metadata is parsed out of a format using a MetadataParser
pub trait MetadataParser {
    fn from_file(location: &Path) -> Option<TrackMetadata>;
}

// and written to a file using a metadata writer
type WriteResult = Option<()>;
pub trait MetadataWriter {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult;
}

pub struct GenericAudioFile;

impl MetadataParser for GenericAudioFile {
    // parse a generic file using libtag
    fn from_file(location: &Path) -> Option<TrackMetadata> {
        let location = location.canonicalize().ok()?;
        info!("Reading tag from location {:?}", location);
        let tagf = TagLibFile::new(&location);
        info!("Got tag: {:?}", tagf);
        let tagf = tagf.ok()?;
        let name = tagf.tag().title().ok()?;
        info!("Got name: [{:?}]", name);
        let bpm = tagf.tag().bpm();
        info!("Got bpm: [{:?}]", bpm);
        let bpm = match bpm {
            Some(b) => Some(b as i64),
            None => None,
        };
        let comment = tagf.tag().comment();
        info!("Got comment: [{:?}]", comment);
        let comment = match comment {
            Ok(s) => Some(vec![s]),
            Err(e) => {
                error!("Got error: {:?}, decoding the comment", e);
                return None;
            }
        };

        Some(TrackMetadata {
            name: name,
            bpm: bpm,
            comments: comment,
        })
    }
}

impl MetadataWriter for GenericAudioFile {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult {
        let location = location.canonicalize().ok()?;
        info!("Reading tag from location {:?}", location);
        let tagf = TagLibFile::new(&location);
        info!("Got tag: {:?}", tagf);
        let tagf = tagf.ok()?;

        let comment = tagf.tag().comment();
        info!("Got comment: [{:?}]", comment);
        let comment = match comment {
            Ok(s) => s,
            Err(e) => {
                error!("Got error: {:?}, decoding the comment", e);
                return None;
            }
        };
        let updated_comment = match ed.update_data(&comment) { 
            Some(c) => c, 
            None => { 
                error!("Got error while updating the comment");
                return None;
            }
        };
        match tagf.tag().set_comment(&updated_comment) { 
            Ok(_) => info!("Successfully set comment"),
            Err(e) => 
            {
                error!("Got error: {:?}, while setting the comment", e);
                return None;
            },
        };
        match tagf.save() {
            Ok(()) => {
                info!("Successfully saved comment to file");
                return Some(());
            },
            Err(e) => 
            {
                error!("Got error {:?}, while saving audio file", e);
                return None;
            }
        }
    }
}
