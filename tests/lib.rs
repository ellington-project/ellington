extern crate ellington;
extern crate assert_cmd;


use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn no_args() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.assert().success();
}
