use taglib::*;

use library::ellingtondata::EllingtonData;
use library::trackmetadata::*;
use std::path::Path;

pub struct GenericTaglibAudioFile;

impl MetadataParser for GenericTaglibAudioFile {
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

impl MetadataWriter for GenericTaglibAudioFile {
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
            Err(e) => {
                error!("Got error: {:?}, while setting the comment", e);
                return None;
            }
        };
        match tagf.save() {
            Ok(()) => {
                info!("Successfully saved comment to file");
                return Some(());
            }
            Err(e) => {
                error!("Got error {:?}, while saving audio file", e);
                return None;
            }
        }
    }
}
