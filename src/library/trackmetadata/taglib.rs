use taglib::*;

use library::ellingtondata::EllingtonData;
use library::trackmetadata::*;
use std::path::Path;

pub struct GenericTaglibAudioFile;

impl MetadataParser for GenericTaglibAudioFile {
    fn parse_title(_line: &String) -> Option<String> {
        unimplemented!()
    }
    fn parse_bpm(_line: &String) -> Option<i64> {
        unimplemented!()
    }
    fn parse_comment(_line: &String) -> Option<String> {
        unimplemented!()
    }
    // parse a generic file using libtag
    fn from_file(location: &Path) -> Option<TrackMetadata> {
        unimplemented!()
        // let location = location.canonicalize().ok()?;
        // info!("Reading tag from location {:?}", location);
        // let tagf = TagLibFile::new(&location);
        // info!("Got tag: {:?}", tagf);
        // let tagf = tagf.ok()?;
        // let name = tagf.tag().title().ok()?;
        // info!("Got name: [{:?}]", name);
        // let bpm = tagf.tag().bpm();
        // info!("Got bpm: [{:?}]", bpm);
        // let bpm = match bpm {
        //     Some(b) => Some(b as i64),
        //     None => None,
        // };
        // let comment = tagf.tag().comment();
        // info!("Got comment: [{:?}]", comment);
        // let comment = match comment {
        //     Ok(s) => Some(vec![s]),
        //     Err(e) => {
        //         error!("Got error: {:?}, decoding the comment", e);
        //         return None;
        //     }
        // };

        // Some(TrackMetadata {
        //     name: name,
        //     bpm: bpm,
        //     comments: comment,
        // })
    }
}

impl MetadataWriter for GenericTaglibAudioFile {
    fn write_ellington_data(_location: &Path, _ed: &EllingtonData) -> WriteResult {
        unimplemented!()
        // let location = location.canonicalize().ok()?;
        // info!("Reading tag from location {:?}", location);
        // let tagf = TagLibFile::new(&location);
        // info!("Got tag: {:?}", tagf);
        // let tagf = tagf.ok()?;

        // let comment = tagf.tag().comment();
        // info!("Got comment: [{:?}]", comment);
        // let comment = match comment {
        //     Ok(s) => s,
        //     Err(e) => {
        //         error!("Got error: {:?}, decoding the comment", e);
        //         return None;
        //     }
        // };
        // let updated_comment = match ed.update_data(&comment) {
        //     Ok(c) => c,

        //         Err(UpdateError::NoDataInComment) => {
        //             warn!("No ellington data in comment");
        //         },
        //         _ => error!("Some other ellington error! This should never be reached!")
        // };
        // match tagf.tag().set_comment(&updated_comment) {
        //     Ok(_) => info!("Successfully set comment"),
        //     Err(e) => {
        //         error!("Got error: {:?}, while setting the comment", e);
        //         return None;
        //     }
        // };
        // match tagf.save() {
        //     Ok(()) => {
        //         info!("Successfully saved comment to file");
        //         return Some(());
        //     }
        //     Err(e) => {
        //         error!("Got error {:?}, while saving audio file", e);
        //         return None;
        //     }
        // }
    }

    fn clear_ellington_data(_location: &Path) -> WriteResult {
        unimplemented!()
    }
}
