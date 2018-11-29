use super::generic::*;
use std::io::Result;
use std::path::PathBuf;
use std::process::Child;

// use flame;

use std::process::Stdio;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Channels {
    Mono,
    Stereo,
}

impl ShellArg for Channels {
    fn flag() -> &'static str {
        "-ac"
    }

    fn value(self: &Channels) -> &'static str {
        match self {
            Channels::Mono => "1",
            Channels::Stereo => "2",
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Format {
    F32le,
}

impl ShellArg for Format {
    fn flag() -> &'static str {
        "-f"
    }

    fn value(self: &Format) -> &'static str {
        match self {
            Format::F32le => "f32le",
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Codec {
    PcmF32le,
}

impl ShellArg for Codec {
    fn flag() -> &'static str {
        "-acodec"
    }

    fn value(self: &Codec) -> &'static str {
        match self {
            Codec::PcmF32le => "pcm_f32le",
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SampleRate {
    Ffo,
    FortyTwoK,
}

impl ShellArg for SampleRate {
    fn flag() -> &'static str {
        "-ar"
    }
    fn value(self: &SampleRate) -> &'static str {
        match self {
            SampleRate::Ffo => "44100",
            SampleRate::FortyTwoK => "42000",
        }
    }
}

#[derive(Debug)]
pub struct FfmpegCommand {
    pub filename: EscapedFilename,
    pub samplerate: SampleRate,
    pub channels: Channels,
    pub format: Format,
    pub codec: Codec,
}

impl FfmpegCommand {
    pub fn default(filename: &PathBuf) -> FfmpegCommand {
        FfmpegCommand {
            filename: EscapedFilename::new(filename),
            samplerate: SampleRate::Ffo,
            channels: Channels::Mono,
            codec: Codec::PcmF32le,
            format: Format::F32le,
        }
    }

    pub fn spawn<'a>(self: &Self) -> Result<Child> {
        // let child =
        self.call().stdout(Stdio::piped()).spawn()
        // .expect("Failed to execute standalone ffmpeg call");

        // child
    }

    // pub fn run_with(self: &Self, )
}

impl ShellProgram for FfmpegCommand {
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
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}
