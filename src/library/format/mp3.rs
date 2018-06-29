use id3::Tag;

use std::fmt;

use library::track::Track;

use std::path::PathBuf;

use library::track::TrackImpl;

// pub struct TrackImpl {
//     pub location: PathBuf,
//     // metadata. Should this be handled separately?
//     // these are from the "file", as in, read, from the file metadata
//     pub name: String,
//     pub bpm: Option<i64>,        // we might not have a bpm value
//     pub comment: Option<String>, // or a comment!
//     // we also have ellington metadata, that we want to manage
//     pub metadata: Option<EllingtonData>,
// }

#[derive(Debug)]
pub struct Mp3(TrackImpl);

impl fmt::Display for Mp3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // All this formatting might be slow...
        let bpm_s = match self.bpm() {
            Some(bpm) => format!("{:?}", bpm),
            None => " - ".to_string(),
        };

        write!(
            f,
            "(name: {} // bpm: {} // loc : [...])",
            bpm_s,
            self.name().unwrap()
        )
    }
}

impl Track for Mp3 {
    fn location(self: &Mp3) -> PathBuf {
        self.0.location.clone()
    }
    fn name(self: &Mp3) -> Option<String> {
        Some(self.0.name.clone())
    }

    fn bpm(self: &Mp3) -> Option<i64> {
        self.0.bpm
    }

    fn comment(self: &Mp3) -> Option<String> {
        self.0.comment.clone()
    }

    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>> {
        let location = path.canonicalize().ok()?;
        let tag = Tag::read_from_path(path).ok()?;
        let name = tag.title()?;
        let bpm_s = &tag.get("TBPM").unwrap().content().text().unwrap();
        let bpm = bpm_s.parse::<i64>().ok()?;

        Some(Box::new(Mp3(TrackImpl {
            location: location.to_path_buf(),
            name: name.to_string(),
            bpm: Some(bpm),
            comment: None,
            metadata: None,
        })))
    }
}
