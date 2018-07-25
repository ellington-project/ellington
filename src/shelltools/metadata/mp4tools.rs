use shelltools::generic::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Mp4Info {
    pub filename: EscapedFilename,
}

impl Mp4Info {
    pub fn new(filename: &PathBuf) -> Mp4Info {
        Mp4Info {
            filename: EscapedFilename::new(filename),
        }
    }
}

impl ShellProgram for Mp4Info {
    const COMMAND_NAME: &'static str = "mp4info";

    fn as_args(self: &Mp4Info) -> Vec<String> {
        vec![self.filename.filename.clone()]
    }
}

#[derive(Debug)]
pub struct Mp4TagsWriteComment {
    pub filename: EscapedFilename,
    pub comment: String,
}

impl Mp4TagsWriteComment {
    pub fn new(filename: &PathBuf, comment: String) -> Mp4TagsWriteComment {
        Mp4TagsWriteComment {
            filename: EscapedFilename::new(filename),
            comment: comment,
        }
    }
}

impl ShellProgram for Mp4TagsWriteComment {
    const COMMAND_NAME: &'static str = "mp4tags";

    fn as_args(self: &Mp4TagsWriteComment) -> Vec<String> {
        vec![
            "-c".to_string(),
            self.comment.clone(),
            self.filename.filename.clone(),
        ]
    }
}
