use shelltools::generic::*;

#[derive(Debug)]
pub struct Id3v2ReadMetadata {
    pub filename: EscapedFilename,
}

impl ShellProgram for Id3v2ReadMetadata {
    const COMMAND_NAME: &'static str = "id3v2";

    fn as_args(self: &Id3v2ReadMetadata) -> Vec<String> { 
        vec![
            "--list".to_string(), 
            self.filename.filename.clone(),
        ]
    }
}

#[derive(Debug)] 
pub struct Id3v2WriteComment {
    pub filename: EscapedFilename, 
    pub description: String, 
    pub comment: String, 
    pub lang: String,
}

impl Id3v2WriteComment {
    fn comment_arg(self: &Self) -> String { 
        format!("{}:{}:{}", self.description, self.comment, self.lang)
    }
}

impl ShellProgram for Id3v2WriteComment {
    const COMMAND_NAME: &'static str = "id3v2";

    fn as_args()(self: &Id3v2WriteComment) -> Vec<String> {
        vec![
            "--comment".to_string(), 
            self.data.as_arg(), 
            self.filename.filename.clone(),
        ]
    }
}