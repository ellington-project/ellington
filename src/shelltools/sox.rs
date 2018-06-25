use std::process::ChildStdout;
use std::io::Read;
use super::generic::*;

use flame;

use itunes::track::Track;
use std::path::Path;
use std::process::Command;

use std::process::Stdio;


#[derive(Debug)]
pub enum Channels {
    Mono,
    Stereo,
}

impl ShellArg for Channels {
    fn flag() -> &'static str {
        "-c"
    }

    fn value(self: &Channels) -> &'static str {
        match self {
            Channels::Mono => "1",
            Channels::Stereo => "2",
        }
    }
}

#[derive(Debug)]
pub enum Encoding {
    Float,
    SignedInteger,
    UnsignedInteger,
    // there are more, but we can ignore them
}

impl ShellArg for Encoding {
    fn flag() -> &'static str {
        "-e"
    }
    fn value(self: &Encoding) -> &'static str {
        match self {
            Encoding::Float => "float",
            Encoding::SignedInteger => "si",
            Encoding::UnsignedInteger => "un",
        }
    }
}

// bits per sample
#[derive(Debug)]
pub enum Bits {
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl ShellArg for Bits {
    fn flag() -> &'static str {
        "-b"
    }
    fn value(self: &Bits) -> &'static str {
        match self {
            Bits::Eight => "8",
            Bits::Sixteen => "16",
            Bits::ThirtyTwo => "32",
        }
    }
}

#[derive(Debug)]
pub enum SampleRate {
    Ffo,
    FortyTwoK,
}

impl ShellArg for SampleRate {
    fn flag() -> &'static str {
        "-r"
    }
    fn value(self: &SampleRate) -> &'static str {
        match self {
            SampleRate::Ffo => "44100",
            SampleRate::FortyTwoK => "42000",
        }
    }
}

#[derive(Debug)]
pub struct SoxCall {
    pub filename: EscapedFilename,
    pub samplerate: SampleRate,
    pub channels: Channels,
    pub encoding: Encoding,
    pub bits: Bits,
}

impl SoxCall {
    pub fn default(filename: EscapedFilename) -> SoxCall {
        SoxCall {
            filename: filename,
            samplerate: SampleRate::Ffo,
            channels: Channels::Mono,
            encoding: Encoding::Float,
            bits: Bits::Sixteen,
        }
    }

    #[flame]
    pub fn run(self: &SoxCall) -> ChildStdout {
        flame::start("spawn call");
        let child = self.call()
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute standalone sox call");
        flame::end("spawn call");

        child.stdout.unwrap()
    }
}

impl ShellProgram for SoxCall {
    const COMMAND_NAME: &'static str = "sox";

    fn as_args(self: &SoxCall) -> Vec<String> {
        vec![
            // the first sox argument is the input filename
            "-V1",
            self.filename.filename.as_str(),
            // next, our output sample rate
            SampleRate::flag(),
            self.samplerate.value(),
            // then the output encoding
            Encoding::flag(),
            self.encoding.value(),
            // then the number of channels in the output
            Channels::flag(),
            self.channels.value(),
            // then the number of bits in the output samples
            Bits::flag(),
            self.bits.value(),
            // penultimately, specify it as raw outputs
            "-t",
            "raw",
            // and finally, specify that we want the command to write to stdout
            "-",
        ].iter()
            .map(|s| s.to_string())
            .collect()
    }
}