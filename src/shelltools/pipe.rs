use super::generic::*;
use std::process::Command;

// Letting us pipe between commands...
#[derive(Debug)]
pub struct PipeCommand<'a, P: 'a + ShellProgram, Q: 'a + ShellProgram> {
    pub source: &'a P,
    pub sink: &'a Q,
}

impl<'a, P: 'a + ShellProgram, Q: 'a + ShellProgram> PipeCommand<'a, P, Q> {
    pub fn as_args(self: &'a PipeCommand<'a, P, Q>) -> Vec<String> {
        let mut source_call = self.source.as_shell_args();
        let mut sink_call = self.sink.as_shell_args();
        let mut pipe = vec!["|".to_string()];
        let mut call = Vec::new();
        call.append(&mut source_call);
        call.append(&mut pipe);
        call.append(&mut sink_call);
        call
    }

    pub fn call_with_sh(self: &'a PipeCommand<'a, P, Q>) -> Command {
        // concatenate the commands into a string...
        let string_command = self
            .as_args()
            .iter()
            .fold("".to_string(), |acc, x| acc + " " + x);
        let mut command = Command::new("sh");
        command.arg("-c");
        command.arg(string_command);
        return command;
    }
}
