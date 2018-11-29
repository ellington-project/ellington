/*
    Officially deprecated, but useful in case anyone wants to use the
    "ShellProgram" infrastructure to call other programs to calculate BPM
    information!
*/
use shelltools::pipe::PipeCommand;
use std::path::PathBuf;

use super::generic::*;
use super::sox::*;

use std::num::ParseFloatError;

#[derive(Debug)]
pub struct BpmCall {
    pub maxbpm: f32,
    pub minbpm: f32,
}

#[allow(dead_code)]
impl BpmCall {
    pub fn default() -> BpmCall {
        BpmCall {
            maxbpm: 450.0, // set sensible defaults for swing music
            minbpm: 50.0,
        }
    }
}

impl ShellProgram for BpmCall {
    // update this for whatever system we're on
    const COMMAND_NAME: &'static str = "tools/bpm-tools/bpm";

    fn as_args(self: &BpmCall) -> Vec<String> {
        vec![
            "-x".to_string(),
            self.maxbpm.to_string(),
            "-m".to_string(),
            self.minbpm.to_string(),
        ]
    }
}

// #[flame]
#[allow(dead_code)]
pub fn bpm_track<T>(location: &PathBuf) -> Result<f64, ParseFloatError> {
    // pipe together a sox and a bpm call
    let overall_call = PipeCommand {
        source: &SoxCommand::default(location),
        sink: &BpmCall::default(),
    };

    let res = overall_call
        .call_with_sh()
        .output()
        .expect("failed to execute process!");

    let result = String::from_utf8_lossy(&res.stdout).replace("\n", "");

    return result.parse::<f64>();
}
