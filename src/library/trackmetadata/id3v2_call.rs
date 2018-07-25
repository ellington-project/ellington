pub struct Id3v2Call;

// use library::ellingtondata::*;
use library::trackmetadata::*;
use regex::Regex;
// use std::collections::BTreeMap;
use shelltools::generic::*;
use shelltools::metadata::id3v2::*;
use std::path::Path;

// An id3v2 comment is of the form (from id3v2 --list) when setting
// -c,  --comment      "DESCRIPTION":"COMMENT":"LANGUAGE"
//                      Set the comment information (both
//                      description and language optional)
// And when reading, it is of the form:
// COMM (Comments): (<desc>)[<lang>]: <comment>

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
    fn parse_title(line: &String) -> Option<String> {
        lazy_static! {
            static ref TITLE_REGEX: Regex = Regex::new(r"TIT2.*: (.*)").unwrap();
        }

        TITLE_REGEX
            .captures(&line)
            .and_then(|captures| captures.get(1))
            .and_then(|rmatch| {
                let title = rmatch.as_str().to_string();
                info!("Successfully parsed track title: {:?}", title);
                Some(title)
            })
    }
    fn parse_bpm(line: &String) -> Option<i64> {
        lazy_static! {
            static ref BPM_REGEX: Regex = Regex::new(r"TBPM.*: (\d+)").unwrap();
        }

        BPM_REGEX
            .captures(&line)
            .and_then(|captures| captures.get(1))
            .and_then(|rmatch| {
                let bpm_str = rmatch.as_str().to_string();
                info!("Found bpm string: {:?}", bpm_str);
                bpm_str.parse::<i64>().ok()
            })
    }
    fn parse_comment(line: &String) -> Option<String> {
        Id3v2Comment::parse(line).and_then(|c| Some(c.comment))
    }
    // parse a generic file using libtag
    fn from_file(location: &Path) -> Option<TrackMetadata> {
        let (stdout, _stderr) = Id3v2ReadMetadata::new(&location.to_path_buf()).run()?;

        let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
        Self::parse_lines(lines)
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
            info!(
                "Writing comment:\nDesc: {:?}\nLang: {:?}\nComm: {:?}",
                original.description, original.language, original.comment
            );
            match ed.update_data(&original.comment) {
                Some(new) => {
                    info!(
                        "Updated comment from/to:\n\t{:?}\n\t{:?}",
                        original.comment, new
                    );
                    let command = Id3v2WriteComment::new(
                        &location.to_path_buf(),
                        original.description,
                        original.language,
                        new,
                    );
                    info!("Running command: {:?}", command.as_args());
                    info!("Running command: {:?}", command.as_shell_args());
                    // write the new comment
                    match command.run() {
                        Some(_) => info!("Ran call successfully"),
                        None => error!("Failed to run, somehow"),
                    }
                }
                None => {
                    error!("No ellington data in comment, or some other error.");
                }
            }
        }

        Some(())
    }

    fn clear_ellington_data(location: &Path) -> WriteResult { 
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
            info!(
                "Writing comment:\nDesc: {:?}\nLang: {:?}\nComm: {:?}",
                original.description, original.language, original.comment
            );
            match EllingtonData::clear_data(&original.comment) {
                Some(new) => {
                    info!(
                        "Updated comment from/to:\n\t{:?}\n\t{:?}",
                        original.comment, new
                    );
                    let command = Id3v2WriteComment::new(
                        &location.to_path_buf(),
                        original.description,
                        original.language,
                        new,
                    );
                    info!("Running command: {:?}", command.as_args());
                    info!("Running command: {:?}", command.as_shell_args());
                    // write the new comment
                    match command.run() {
                        Some(_) => info!("Ran call successfully"),
                        None => error!("Failed to run, somehow"),
                    }
                }
                None => {
                    error!("No ellington data in comment, or some other error.");
                }
            }
        }

        Some(())
    }
}
