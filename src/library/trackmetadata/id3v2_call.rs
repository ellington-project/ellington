pub struct Id3v2Call;

// use library::ellingtondata::*;
use library::trackmetadata::*;
use regex::Regex;
// use std::collections::BTreeMap;
use std::path::Path;
use shelltools::metadata::id3v2::*;
use shelltools::generic::*;

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

        info!("Found comment: {:?}/{:?}/{:?}", c.description, c.language, c.comment);

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
        let mut bpm : Option<i64> = None; 
        let mut comments: Option<Vec<String>> = None; 

        for line in lines {
            // try and parse a title
            match TITLE_REGEX
                .captures(&line)
                .and_then(|captures| {
                    captures.get(1)
                })
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
                .and_then(|captures| {
                    captures.get(1)
                })
                .and_then(|rmatch| {
                    let bpm_str = rmatch.as_str().to_string();
                    info!("Found bpm string: {:?}", bpm_str);
                    bpm_str.parse::<i64>().ok()
                }).and_then(|bpm_i| {
                    bpm = Some(bpm_i);
                    Some(())
                }){
                Some(_) => continue,
                _ => {}
            };

            // try and parse a comment
            match Id3v2Comment::parse(&line).and_then(|id3v2c|{
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
            Some(name) => { 
                Some(TrackMetadata {
                    name: name, 
                    bpm: bpm, 
                    comments: comments
                })
            }, 
            None => None
        }
    }
}

impl MetadataWriter for Id3v2Call {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult {
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
        //     Some(c) => c,
        //     None => {
        //         error!("Got error while updating the comment");
        //         return None;
        //     }
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
}
