use shelltools::ffmpeg::FfmpegCommand;
use std::path::PathBuf;

pub mod algorithms;
pub mod sources;

use self::algorithms::naive::Naive;
use self::sources::audiostream::AudioStream;

pub trait TempoEstimator {
    const NAME: &'static str;
    fn run(audio_file: &PathBuf) -> Option<i64>;
}

pub struct FfmpegNaiveTempoEstimator {}

impl TempoEstimator for FfmpegNaiveTempoEstimator {
    const NAME: &'static str = "ffmpeg-naive";
    fn run(audio_file: &PathBuf) -> Option<i64> {
        let call = FfmpegCommand::default(&audio_file);
        let mut child = match call.spawn() {
            Err(e) => {
                error!(
                    "Failed to run ffmpeg for audio file {:?}, with io error {:?}",
                    audio_file, e
                );
                None
            }
            Ok(c) => Some(c),
        }?;

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

        match child.wait() {
            Err(e) => {
                error!(
                    "Failed to wait on ffmpeg child for audio file {:?}, with io error {:?}",
                    audio_file, e
                );
                None
            }
            Ok(_) => result,
        }
    }
}


pub struct BellsonTempoEstimator {} 

impl TempoEstimator for BellsonTempoEstimator {
    const NAME: &'static str = "bellson";
    fn run(_audio_file: &PathBuf) -> Option<i64> {
        Some(0)
        // let call = BellsonCommand::default(&audio_file); 
        // let mut child = match call.run() { 
        //     Err(e) => {
        //         error!(
        //             "Failed to run bellson for audio file {:?}, with io error {:?}",
        //             audio_file, e
        //         );
        //         None
        //     }
        //     Ok(c) => Some(c),
        // }?;

        // let result = {
        //     let bellson_stream = match &mut child.stdout {
        //         Some(s) => Some(AudioStream::from_stream(s)),
        //         None => {
        //             error!("Ffmpeg stream did not run properly!");
        //             None
        //         }
        //     }?;

        //     Some(Naive::default().analyse(ffmpeg_stream) as i64)
        // };

        // match child.wait() {
        //     Err(e) => {
        //         error!(
        //             "Failed to wait on ffmpeg child for audio file {:?}, with io error {:?}",
        //             audio_file, e
        //         );
        //         None
        //     }
        //     Ok(_) => result,
        // }
    }
}