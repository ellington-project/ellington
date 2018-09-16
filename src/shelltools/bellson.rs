use super::generic::*;
use std::io::Result;
use std::path::PathBuf;
use std::process::Child;

// use flame;

use std::process::Stdio;

#[derive(Debug)]
pub struct BellsonCommand {
    pub path: &PathBuf,
    pub model: &PathBuf
}

impl BellsonCommand {
    pub fn default(path: &PathBuf, model: &PathBuf) -> BellsonCommand {
        BellsonCommand {
            path: path, 
            model: model
        }
    }
}

impl ShellProgram for BellsonCommand {
    const COMMAND_NAME: &'static str = "ffmpeg";

    fn as_args(self: &Self) -> Vec<String> {
        vec![
            "-loglevel",
            "quiet",
            // the first ffmpeg argument is the input filename
            "-i",
            self.filename.filename.as_str(),
            // next, the format of the output,
            Format::flag(),
            self.format.value(),
            // then the codec
            Codec::flag(),
            self.codec.value(),
            // then the number of channels in the output
            Channels::flag(),
            self.channels.value(),
            //  our output sample rate
            SampleRate::flag(),
            self.samplerate.value(),
            // finally, tell ffmpeg to write to stdout
            "pipe:1",
        ].iter()
            .map(|s| s.to_string())
            .collect()
    }
}
