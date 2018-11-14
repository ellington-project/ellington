use regex::Regex;
use shelltools::bellson::BellsonCommand;
use shelltools::ffmpeg::FfmpegCommand;
use shelltools::generic::ShellProgram;
use std::path::PathBuf;
use types::AlgorithmE;

pub mod algorithms;
pub mod sources;

use self::algorithms::naive::Naive;
use self::sources::audiostream::AudioStream;

pub trait TempoEstimator {
    const ALGORITHM: AlgorithmE;
    fn run(audio_file: &PathBuf) -> Option<i64>;
}

// todo: find a better way of doing this! I assume this is slow?
pub fn run_estimator(name: &str, audio_file: &PathBuf) -> Option<(i64, &'static str)> {
    match name {
        "naive" => FfmpegNaiveTempoEstimator::run(audio_file)
            .and_then(|bpm| Some((bpm, FfmpegNaiveTempoEstimator::ALGORITHM.print()))),
        "bellson" => BellsonTempoEstimator::run(audio_file)
            .and_then(|bpm| Some((bpm, BellsonTempoEstimator::ALGORITHM.print()))),
        _ => {
            error!("Could not find a tempo estimator of that name!");
            None
        }
    }
}

pub struct FfmpegNaiveTempoEstimator {}

impl TempoEstimator for FfmpegNaiveTempoEstimator {
    const ALGORITHM: AlgorithmE = AlgorithmE::Naive;
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
    const ALGORITHM: AlgorithmE = AlgorithmE::Bellson;
    fn run(audio_file: &PathBuf) -> Option<i64> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Mean: (\d+)").unwrap();
        }
        let call = BellsonCommand::default(audio_file);
        match call.run() {
            Some((stdout, _stderr)) => {
                let captures = RE.captures(stdout.as_str())?;

                let bpm = captures.get(1)?.as_str();

                let bpm = bpm.parse::<i64>().ok()?;

                Some(bpm)
            }
            _ => {
                error!("Got error while running bellson!");
                None
            }
        }
    }
}
