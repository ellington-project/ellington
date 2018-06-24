use audio_in::AudioBuffer;
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

#[flame]
pub fn call_sox_and_read_f32(track: &Track) -> AudioBuffer {
    flame::start("spawn call");
    let standalone = SoxCall::default(track.escaped_location())
        .call()
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute standalone sox call");
        flame::end("spawn call");

    AudioBuffer::from_stream(standalone.stdout.unwrap())
}

#[flame]
pub fn run_sox_and_read_file(mp3: &String, dat: &String) -> AudioBuffer {
    // Get the data using the sox command
    let command = format!(
        // "sox -V1 \"{:?}\" -L -r 48000 -e float -b 16 -t raw \"{:?}\"",
        "sox -V1 {} -r 44100 -e float -c 1 -b 16 -t raw {}",
        EscapedFilename::new(mp3).filename, EscapedFilename::new(dat).filename
    );

    flame::start("run raw sox command");
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    flame::end("run raw sox command");

    assert!(output.status.success());

    AudioBuffer::from_file(dat)
}

#[flame]
pub fn test_sox_calls_equal(track: &Track) -> () {
    // first with a call to a file...
    // call sox, and read the data: 
    // first, escape the mp3, and dat filenames:

    let mp3 = &track.location.to_str().unwrap().to_string();
    let dat = Path::new(&mp3).with_extension("txt").to_str().unwrap().to_string();

    let file_buffer = run_sox_and_read_file(&mp3, &dat);

    // now, run the equivalent sox pipe call
    let pipe_buffer = call_sox_and_read_f32(track);

    let AudioBuffer(pdata) = pipe_buffer;
    let AudioBuffer(fdata) = file_buffer;

    flame::start("comparing results");
    assert_eq!(pdata, fdata);
    flame::end("comparing results");

}
