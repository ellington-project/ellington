use super::generic::*;
use std::io::Result;
use std::path::PathBuf;
use std::process::Child;

// use flame;

use std::process::Stdio;

#[derive(Debug)]
pub struct BellsonCommand {
    pub path: EscapedFilename,
}

impl BellsonCommand {
    pub fn default(path: &PathBuf) -> BellsonCommand {
        BellsonCommand {
            path: EscapedFilename::new(path),
        }
    }
}

impl ShellProgram for BellsonCommand {
    const COMMAND_NAME: &'static str = "bellson-infer";

    fn as_args(self: &Self) -> Vec<String> {
        vec![self.path.filename.as_str()]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}
