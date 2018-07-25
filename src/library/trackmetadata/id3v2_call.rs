pub struct Id3v2Call;

// use library::ellingtondata::*;
use library::trackmetadata::*;
use regex::Regex;
// use std::collections::BTreeMap;
use shelltools::generic::*;
use shelltools::metadata::id3v2::*;
use std::path::Path;

#[derive(Debug)]
struct Id3v2Comment {
    description: String,
    language: String,
    comment: String,
}

impl Id3v2Comment {
    pub fn parse(line: &String) -> Option<Id3v2Comment> {
        lazy_static! {
            static ref COMMENT_REGEX: Regex =
                Regex::new(r"COMM \(Comments\): \((.*)\)\[(\S{0,3})\]: (.*)").unwrap();
        }

        let mut c = Id3v2Comment {
            description: String::new(),
            language: String::new(),
            comment: String::new(),
        };

        let captures = COMMENT_REGEX.captures(line)?;

        c.description = captures.get(1)?.as_str().to_string();
        c.language = captures.get(2)?.as_str().to_string();
        c.comment = captures.get(3)?.as_str().to_string();

        info!(
            "Found comment: {:?}/{:?}/{:?}",
            c.description, c.language, c.comment
        );

        Some(c)
    }
}

impl MetadataParser for Id3v2Call {
    // parse a generic file using libtag
    fn from_file(location: &Path) -> Option<TrackMetadata> {
        lazy_static! {
            static ref BPM_REGEX: Regex = Regex::new(r"TBPM.*: (\d+)").unwrap();
            static ref TITLE_REGEX: Regex = Regex::new(r"TIT2.*: (.*)").unwrap();
            // static ref ALBUM_REGEX: Regex = Regex::new(r"TALB.*\: (.*)").unwrap();
        }

        let (stdout, _stderr) = Id3v2ReadMetadata::new(&location.to_path_buf()).run()?;

        let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

        let mut name: Option<String> = None;
        let mut bpm: Option<i64> = None;
        let mut comments: Option<Vec<String>> = None;

        for line in lines {
            // try and parse a title
            match TITLE_REGEX
                .captures(&line)
                .and_then(|captures| captures.get(1))
                .and_then(|rmatch| {
                    name = Some(rmatch.as_str().to_string());
                    info!("Successfully parsed track title: {:?}", name);
                    Some(())
                }) {
                Some(_) => continue,
                _ => {}
            };

            // try and parse a bpm
            match BPM_REGEX
                .captures(&line)
                .and_then(|captures| captures.get(1))
                .and_then(|rmatch| {
                    let bpm_str = rmatch.as_str().to_string();
                    info!("Found bpm string: {:?}", bpm_str);
                    bpm_str.parse::<i64>().ok()
                })
                .and_then(|bpm_i| {
                    bpm = Some(bpm_i);
                    Some(())
                }) {
                Some(_) => continue,
                _ => {}
            };

            // try and parse a comment
            match Id3v2Comment::parse(&line).and_then(|id3v2c| {
                if !comments.is_some() {
                    comments = Some(Vec::new());
                }
                match comments.as_mut() {
                    Some(arr) => arr.push(id3v2c.comment),
                    None => {}
                }
                Some(())
            }) {
                Some(_) => continue,
                None => {}
            }
        }

        match name {
            Some(name) => Some(TrackMetadata {
                name: name,
                bpm: bpm,
                comments: comments,
            }),
            None => None,
        }
    }
}

impl MetadataWriter for Id3v2Call {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult {
        // Parse the file to get a list of comments, as id3v2 comments

        let (stdout, _stderr) = Id3v2ReadMetadata::new(&location.to_path_buf()).run()?;

        // map across the lines, and try to turn them each into a comment...
        let comments: Vec<Id3v2Comment> = stdout
            .lines()
            .map(|s| s.to_string())
            .filter_map(|line| Id3v2Comment::parse(&line))
            .collect();

        // for each of the comments, try to write an updated form of that comment to the file
        for original in comments {
            // try to update the comment
            match ed.update_data(&original.comment) {
                Some(new) => {
                    info!(
                        "Updated comment from/to:\n\t{:?}\n\t{:?}",
                        original.comment, new
                    );
                    // write the new comment
                    match Id3v2WriteComment::new(
                        &location.to_path_buf(),
                        original.description,
                        original.language,
                        original.comment,
                    ).run()
                    {
                        Some(_) => info!("Ran call successfully"),
                        None => info!("Failed to run, somehow"),
                    }
                }
                None => {
                    info!("No ellington data in comment, or some other error.");
                }
            }
        }

        Some(())
    }
}
