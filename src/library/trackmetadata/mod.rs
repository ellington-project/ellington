use taglib::*;

use std::path::Path;

// a structure storing metadata about some track, in a format agnostic manner
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMetadata {
    pub name: String,                  // we must always have a track name
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

// metadata is parsed out of a format using a MetadataParser
pub trait MetadataParser {
    fn from_file(location: &Path) -> Option<TrackMetadata>;
}

// and written to a file using a metadata writer
type WriteResult = Option<()>;
pub trait MetadataWriter {
    fn update_comments(location: &Path, f: &Fn(String) -> String) -> WriteResult;
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
    fn update_comments(location: &Path, f: &Fn(String) -> String) -> WriteResult {
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
        let updated = f(comment);
        match tagf.tag().set_comment(&updated) {
            Ok(_) => Some(()),
            Err(_) => None
        }
    }
}
