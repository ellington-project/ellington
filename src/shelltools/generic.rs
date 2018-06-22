#![feature(associated_constants)]
use std::process::Command;

#[derive(Debug)]
pub struct EscapedFilename {
    pub filename: String,
}

impl EscapedFilename {
    pub fn new(unescaped: &String) -> EscapedFilename {
        EscapedFilename {
            filename: unescaped
                .replace("%20", " ")
                .replace("file://", "")
                .replace(" ", "\\ ")
                .replace("'", "\\'"),
        }
    }
}

// define a trait for shell argument types
pub trait ShellArg {
    fn flag() -> &'static str;
    fn value(&self) -> &'static str;
}

// and a trait for command line programs
pub trait ShellProgram {
    const COMMAND_NAME: &'static str;

    fn as_args(&self) -> Vec<String>;

    // the below are defined almost entirely in terms of as_args and COMMAND_NAME, so don't need to be defined specially for each program

    fn as_shell_args(&self) -> Vec<String> {
        let mut args = vec![Self::COMMAND_NAME.to_string()];
        args.append(&mut self.as_args());
        return args;
    }

    fn call(&self) -> Command {
        let mut command = Command::new(Self::COMMAND_NAME);
        command.args(self.as_args());
        return command;
    }
}
