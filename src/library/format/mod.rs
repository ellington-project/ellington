use library::ellingtondata::EllingtonData;
use library::track::Track;
use std::fmt;
use std::path::PathBuf;

// re-export the Mp3 format
pub use self::mp3::Mp3;
mod mp3;

// a default "unknown", or erroring file type
#[derive(Debug)]
pub struct UnknownFile {
    location: PathBuf,
    error: &'static str,
}

impl fmt::Display for UnknownFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // All this formatting might be slow...
        write!(
            f,
            "(unknown file, location: {:?}, error: {:?})",
            self.location, self.error
        )
    }
}

impl Track for UnknownFile {
    fn location(self: &UnknownFile) -> PathBuf {
        self.location.clone()
    }
    fn name(self: &UnknownFile) -> Option<String> {
        None
    }

    fn bpm(self: &UnknownFile) -> Option<i64> {
        None
    }

    fn comments(self: &UnknownFile) -> Option<Vec<String>> {
        None
    }

    fn ellington_data(self: &UnknownFile) -> Option<Vec<EllingtonData>> { 
        None
    }

    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>> {
        Some(Box::new(UnknownFile {
            location: path.to_path_buf(),
            error: "Cannot handle files of this type!",
        }))
    }
    fn write_data(self: &Self, _new_data: EllingtonData) -> Option<()>{
        println!("Cannot yet write data for this kind of file!");
        None
    }
}
