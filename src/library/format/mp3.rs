use id3::frame::Comment;
use id3::Tag;
use library::ellingtondata::EllingtonData;
use shelltools::generic::{EscapedFilename, ShellProgram};
use shelltools::id3v2::{Id3Comment, Id3v2Call};

use std::fmt;

use library::track::Track;

use std::path::PathBuf;

use library::track::TrackImpl;

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
            self.name().unwrap(),
            bpm_s
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

    fn comments(self: &Mp3) -> Option<Vec<String>> {
        self.0.comments.clone()
    }

    // #[flame]
    fn ellington_data(self: &Mp3) -> Option<Vec<EllingtonData>> {
        // and the same for ellington data
        match &self.0.comments {
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

    // #[flame]
    fn from_file_impl(path: &PathBuf) -> Option<Box<Track + 'static>> {
        let location = path.canonicalize().ok()?;
        let tag = Tag::read_from_path(path).ok()?;
        let name = tag.title()?;
        let bpm = tag.get("TBPM").and_then(|f| f.content().text()).and_then(|s| s.parse::<i64>().ok());

        // get the list of comments
        let comment_v: Vec<String> = tag.comments().map(|c: &Comment| c.text.clone()).collect();

        let comments = match comment_v.len() {
            0 => None,
            _ => Some(comment_v),
        };

        Some(Box::new(Mp3(TrackImpl {
            location: location.to_path_buf(),
            name: name.to_string(),
            bpm: bpm,
            comments: comments,
        })))
    }

    // #[flame]
    fn write_data(self: &Self, new_data: EllingtonData) -> Option<()> {
        // get a reference to the tag

        let tag = Tag::read_from_path(&self.0.location).ok()?;

        // for each of the comments, change it and write it to the mp3 file
        for comment in tag.comments() {
            // figure out what the new data willl be
            match new_data.update_data(&comment.text) {
                Some(updated) => {
                    let id3c = Id3Comment {
                        description: comment.description.clone(),
                        lang: comment.lang.clone(),
                        comment: updated,
                    };

                    let id3 = Id3v2Call {
                        filename: EscapedFilename::new(&self.0.location),
                        data: id3c,
                    };

                    let res = id3.call().output().expect("failed to execute process!");

                    let out = String::from_utf8_lossy(&res.stdout).replace("\n", "");
                    let err = String::from_utf8_lossy(&res.stderr).replace("\n", "");
                    println!("Call result: {} / {}", out, err);
                }
                None => println!("No ellington data found in comment!"),
            };
        }
        Some(())
    }
}
