use shelltools::ffmpeg::FfmpegCommand;
use std::path::PathBuf;

pub mod algorithms;
pub mod sources;

use self::algorithms::naive::Naive;
use self::sources::audiostream::AudioStream;

pub trait Pipeline {
    const NAME: &'static str;
    fn run(audio_file: &PathBuf) -> Option<i64>;
}

pub struct FfmpegNaivePipeline {}

impl Pipeline for FfmpegNaivePipeline {
    const NAME: &'static str = "ffmpeg-naive";
    fn run(audio_file: &PathBuf) -> Option<i64> {
        let call = FfmpegCommand::default(&audio_file);
        let mut child = call.run();

        let result = {
            let ffmpeg_stream = match &mut child.stdout {
                Some(s) => Some(AudioStream::from_stream(s)),
                None => {
                    error!("Ffmpeg stream did not run properly!");
                    None
                }
            }?;

            Some(Naive::default().analyse(ffmpeg_stream) as i64)
        };

        child.wait().expect("failed to wait on child");

        result
    }
}
