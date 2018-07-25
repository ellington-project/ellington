pub struct Mp4ToolsCall;

// use library::ellingtondata::*;
use library::trackmetadata::*;
use regex::Regex;
// use std::collections::BTreeMap;
use shelltools::generic::*;
use shelltools::metadata::mp4tools::*;
use std::path::Path;

// Example output:
// mp4info version 2.0.0
// /Users/adam/Music/iTunes/iTunes Media/Music/The Big Easy/This Is Hip/01 King Of The Swingers.m4a:
// Track   Type    Info
// 1   audio   alac, 192.973 secs, 706 kbps, 44100 Hz
//  Name: King Of The Swingers
//  Artist: The Big Easy
//  Encoded with: iTunes 12.7.5.9
//  Album: This Is Hip
//  Track: 1 of 10
//  Disk: 1 of 1
//  GenreType: 9, Jazz
//  Grouping: R&R Sherman
//  BPM: 0
//  Comments: command line comment
//  Part of Compilation: no
//  Part of Gapless Album: no
//  Album Artist: The Big Easy

impl MetadataParser for Mp4ToolsCall {
    fn parse_title(line: &String) -> Option<String> {
        lazy_static! {
            static ref TITLE_REGEX: Regex = Regex::new(r" Name: (.*)").unwrap();
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
            static ref BPM_REGEX: Regex = Regex::new(r" BPM: (\d+)").unwrap();
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
        lazy_static! {
            static ref COMMENT_REGEX: Regex = Regex::new(r" Comments: (.*)").unwrap();
        }

        COMMENT_REGEX
            .captures(&line)
            .and_then(|captures| captures.get(1))
            .and_then(|rmatch| {
                let comment = rmatch.as_str().to_string();
                info!("Successfully parsed track comment: {:?}", comment);
                Some(comment)
            })
    }
    // parse a generic file using libtag
    fn from_file(location: &Path) -> Option<TrackMetadata> {
        let (stdout, _stderr) = Mp4Info::new(&location.to_path_buf()).run()?;

        let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

        Self::parse_lines(lines)
    }
}

impl MetadataWriter for Mp4ToolsCall {
    fn write_ellington_data(location: &Path, ed: &EllingtonData) -> WriteResult {
        // Reparse the file to get the comment data
        let original = &Self::from_file(location)?.comments?[0];

        // try to write an updated form of that comment to the file
        match ed.update_data(&original) {
            Some(new) => {
                info!("Updated comment from/to:\n\t{:?}\n\t{:?}", original, new);
                let command = Mp4TagsWriteComment::new(&location.to_path_buf(), new);
                info!("Running command: {:?}", command.as_args());
                info!("Running command: {:?}", command.as_shell_args());
                match command.run() {
                    Some(_) => info!("Ran call successfully"),
                    None => error!("Failed to run, somehow"),
                }
            }
            None => {
                error!("No ellington data in comment, or some other error.");
            }
        }

        Some(())
    }
}
