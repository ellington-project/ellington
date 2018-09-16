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

// todo: find a better way of doing this! I assume this is slow?
pub fn run_estimator(name: &str, audio_file: &PathBuf) -> Option<(i64, &'static str)> {
    match name {
        "naive" => FfmpegNaiveTempoEstimator::run(audio_file)
            .and_then(|bpm| Some((bpm, FfmpegNaiveTempoEstimator::NAME))),
        "bellson" => BellsonTempoEstimator::run(audio_file)
            .and_then(|bpm| Some((bpm, BellsonTempoEstimator::NAME))),
        _ => {
            error!("Could not find a tempo estimator of that name!");
            None
        }
    }
}

pub struct FfmpegNaiveTempoEstimator {}

impl TempoEstimator for FfmpegNaiveTempoEstimator {
    const NAME: &'static str = "naive";
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
        None
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
