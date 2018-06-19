use std::process::Command;

// define a trait for sox argument types
trait SoxArg {
    fn flag() -> &'static str;
    fn value(&self) -> &'static str;
}

#[derive(Debug)]
pub enum Channels {
    Mono,
    Stereo 
}

impl SoxArg for Channels {
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

impl SoxArg for Encoding {
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

impl SoxArg for Bits {
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

impl SoxArg for SampleRate {
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
    // empty struct for now
    pub samplerate : u32, 
    pub channels : Channels,
// field: Type
}

impl SoxCall {
    pub fn call<'a>() -> Command {
        let infile = "\"in.mp3\"";
        let outfile = "-";
        let mut command = Command::new("sox");
        command
            .arg("-V1")
            .arg(infile)
            .arg("-r")
            .arg("44100")
            .arg("-e")
            .arg("float")
            .arg("-c")
            .arg("1")
            .arg("-b")
            .arg("16")
            .arg("-t")
            .arg("raw")
            .arg(outfile);

        return command;
    }
}
