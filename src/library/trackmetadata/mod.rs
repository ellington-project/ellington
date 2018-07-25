use library::ellingtondata::*;
use library::filemetadata::*;
use std::collections::BTreeMap;
use std::path::Path;

pub mod id3v2_call;
pub mod taglib;

use self::id3v2_call::*;

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

    pub fn from_file(location: &Path, fmd: &FileMetadata) -> Option<TrackMetadata> {
        match fmd.ftype {
            AudioFileType::Flac => None,
            AudioFileType::M4a => None,
            AudioFileType::M4p => None,
            AudioFileType::Mp3 => Id3v2Call::from_file(location),
            AudioFileType::Mp4 => None,
            AudioFileType::Wav => None,
            AudioFileType::Alac => None,
            AudioFileType::NotAudio => None,
        }
    }

    pub fn write_ellington_data(
        location: &Path,
        fmd: &FileMetadata,
        ed: &EllingtonData,
    ) -> WriteResult {
        match fmd.ftype {
            AudioFileType::Flac => None,
            AudioFileType::M4a => None,
            AudioFileType::M4p => None,
            AudioFileType::Mp3 => Id3v2Call::write_ellington_data(location, ed),
            AudioFileType::Mp4 => None,
            AudioFileType::Wav => None,
            AudioFileType::Alac => None,
            AudioFileType::NotAudio => None,
        }
    }
}

// metadata is parsed out of a format using a MetadataParser
pub trait MetadataParser {
    fn from_file(location: &Path) -> Option<TrackMetadata>;
}

// and written to a file using a metadata writer
pub type WriteResult = Option<()>;
pub trait MetadataWriter {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult;
}
