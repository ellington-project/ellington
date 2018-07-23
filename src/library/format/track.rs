use library::ellingtondata::EllingtonData;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Debug;
use std::path::PathBuf;
use taglib::*;

#[derive(Debug, Clone)]
pub struct TrackImpl {
    pub location: PathBuf,
    // metadata. Should this be handled separately?
    // these are from the "file", as in, read, from the file metadata
    pub name: String,
    pub bpm: Option<i64>,              // we might not have a bpm value
    pub comments: Option<Vec<String>>, // or comments!
}

pub trait Track: fmt::Display + Debug {
    fn location(self: &Self) -> PathBuf;
    fn name(self: &Self) -> Option<String>;
    fn bpm(self: &Self) -> Option<i64>;
    fn comments(self: &Self) -> Option<Vec<String>>;
    fn ellington_data(self: &Self) -> Option<Vec<EllingtonData>>;
    fn write_data(self: &Self, _new_data: EllingtonData) -> Option<()>;
    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>>
    where
        Self: Sized;
}

impl Track {
    pub fn from_file(location: &PathBuf) -> Option<Box<Track + 'static>> {
        match location.extension().and_then(OsStr::to_str) {
            _ => GenericTrack::from_file_impl(location)
            // Some("mp3") => Mp3::from_file_impl(location),
            // Some("mp4") => UnknownFile::from_file_impl(location),
            // Some("m4a") => UnknownFile::from_file_impl(location),
            // Some("flac") => UnknownFile::from_file_impl(location),
            // _ => UnknownFile::from_file_impl(location),
        }
    }
}

#[derive(Debug)]
pub struct GenericTrack {
    pub location: PathBuf,
    pub name: String,
    pub bpm: Option<i64>,
    pub comments: Option<Vec<String>>,
    // track dependent stuff.
    filemetadata: TagLibFile,
}

impl fmt::Display for GenericTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // All this formatting might be slow...
        let bpm_s = match self.bpm() {
            Some(bpm) => format!("{:?}", bpm),
            None => " - ".to_string(),
        };

        write!(
            f,
            "(name: {} // bpm: {} // loc : [...])",
            self.name().unwrap(),
            bpm_s
        )
    }
}

impl Track for GenericTrack {
    fn location(self: &GenericTrack) -> PathBuf {
        self.location.clone()
    }
    fn name(self: &Self) -> Option<String> {
        Some(self.name.clone())
    }
    fn bpm(self: &Self) -> Option<i64> {
        self.bpm
    }
    fn comments(self: &Self) -> Option<Vec<String>> {
        self.comments.clone()
    }
    fn ellington_data(self: &Self) -> Option<Vec<EllingtonData>> {
        match &self.comments {
            Some(cs) => {
                let ed: Vec<EllingtonData> = cs
                    .iter()
                    .flat_map(|c| EllingtonData::parse_data(&c))
                    .collect();
                match ed.len() {
                    0 => None,
                    _ => Some(ed),
                }
            }
            None => None,
        }
    }
    fn write_data(self: &Self, _new_data: EllingtonData) -> Option<()> {
        unimplemented!()
    }
    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>> {
        let location = path.canonicalize().ok()?;
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

        Some(Box::new(GenericTrack {
            location: location,
            name: name,
            bpm: bpm,
            comments: comment,
            // track dependent stuff.
            filemetadata: tagf,
        }))

        // let tag = Tag::read_from_path(path);
        // // print out the tag result, as we can't put logging inside id3lib
        // match &tag {
        //     Ok(t) => info!("Decoded tag for track: [{}]", t.title().unwrap_or("unknown")),
        //     Err(e) => error!("Got tag decoding error: {}", e)
        // };
        // let tag = tag.ok()?;
        // let name = tag.title()?;
        // let bpm = tag.get("TBPM").and_then(|f| f.content().text()).and_then(|s| s.parse::<i64>().ok());

        // // get the list of comments
        // let comment_v: Vec<String> = tag.comments().map(|c: &Comment| c.text.clone()).collect();

        // let comments = match comment_v.len() {
        //     0 => None,
        //     _ => Some(comment_v),
        // };

        // Some(Box::new(Mp3(TrackImpl {
        //     location: location.to_path_buf(),
        //     name: name.to_string(),
        //     bpm: bpm,
        //     comments: comments,
        // })))
    }
}
