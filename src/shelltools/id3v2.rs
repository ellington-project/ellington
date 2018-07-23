
use super::generic::*;

#[derive(Debug)]
pub struct Id3Comment {
    pub description: String, 
    pub comment: String,
    pub lang: String,
}

impl Id3Comment {
    fn as_arg(self: &Self) -> String { 
        format!("{}:{}:{}", self.description, self.comment, self.lang)
    }
}

#[derive(Debug)]
pub struct Id3v2Call {
    pub filename: EscapedFilename,
    pub data: Id3Comment,
}

impl ShellProgram for Id3v2Call {
    // update this for whatever system we're on
    const COMMAND_NAME: &'static str = "id3v2";

    fn as_args(self: &Id3v2Call) -> Vec<String> {
        vec![
            "-c".to_string(),
            self.data.as_arg(),
            self.filename.filename.clone(),
        ]
    }
}
