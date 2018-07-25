use shelltools::generic::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Id3v2ReadMetadata {
    pub filename: EscapedFilename,
}
impl Id3v2ReadMetadata {
    pub fn new(filename: &PathBuf) -> Id3v2ReadMetadata {
        Id3v2ReadMetadata {
            filename: EscapedFilename::new(filename),
        }
    }
}

impl ShellProgram for Id3v2ReadMetadata {
    const COMMAND_NAME: &'static str = "id3v2";

    fn as_args(self: &Id3v2ReadMetadata) -> Vec<String> {
        vec!["--list".to_string(), self.filename.filename.clone()]
    }
}

#[derive(Debug)]
pub struct Id3v2WriteComment {
    pub filename: EscapedFilename,
    pub description: String,
    pub lang: String,
    pub comment: String,
}

impl Id3v2WriteComment {
    pub fn new(
        filename: &PathBuf,
        description: String,
        lang: String,
        comment: String,
    ) -> Id3v2WriteComment {
        Id3v2WriteComment {
            filename: EscapedFilename::new(filename),
            description: description,
            lang: lang,
            comment: comment,
        }
    }

    fn comment_arg(self: &Self) -> String {
        format!("{}:{}:{}", self.description, self.lang, self.comment)
    }
}

impl ShellProgram for Id3v2WriteComment {
    const COMMAND_NAME: &'static str = "id3v2";

    fn as_args(self: &Id3v2WriteComment) -> Vec<String> {
        vec![
            "--comment".to_string(),
            self.comment_arg(),
            self.filename.filename.clone(),
        ]
    }
}
