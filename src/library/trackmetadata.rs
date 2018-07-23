use std::path::Path;

// a structure storing metadata about some track
pub struct TrackMetadata {
    pub name: String,                  // we must always have a track name
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

pub trait MetadataParser {
    fn from_file(location: &Path) -> Option<TrackMetadata>;
}
