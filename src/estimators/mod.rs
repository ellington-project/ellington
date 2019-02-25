use regex::Regex;
use shelltools::bellson::BellsonCommand;
// use shelltools::ffmpeg::FfmpegCommand;
use shelltools::generic::ShellProgram;
use std::path::PathBuf;
use types::AlgorithmE;

// pub mod algorithms;
pub mod sources;

use simple_bpm::*; 
use hodges::*; 

pub trait TempoEstimator {
    const ALGORITHM: AlgorithmE;
    fn run(audio_file: &PathBuf) -> Option<i64>;
}

// todo: find a better way of doing this! I assume this is slow?
#[flame("Generic")]
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
    #[flame("FfmpegNaiveTempoEstimator")]
    fn run(audio_file: &PathBuf) -> Option<i64> {

        let mut estimator = SimpleEstimator::with_accuracy(8); 

        let state: State<&[f32]> =
        State::from_file(audio_file.clone())?;

        Some(estimator.analyse(state.flatten().cloned()) as i64)
    }
}

pub struct BellsonTempoEstimator {}

impl TempoEstimator for BellsonTempoEstimator {
    const ALGORITHM: AlgorithmE = AlgorithmE::Bellson;
    #[flame("BellsonTempoEstimator")]
    fn run(audio_file: &PathBuf) -> Option<i64> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Mean: (\d+)").unwrap();
        }
        let call = BellsonCommand::default(audio_file);
        match call.run() {
            Some((stdout, _stderr)) => {
                let captures = RE.captures(stdout.as_str())?;

                debug!("Captures: {:?}", captures);

                let bpm = captures.get(1)?.as_str();

                debug!("BPM: {:?}", bpm);

                let bpm = bpm.parse::<i64>().ok()?;

                debug!("bpm<i64>: {:?}", bpm);

                Some(bpm)
            }
            _ => {
                error!("Got error while running bellson!");
                None
            }
        }
    }
}
