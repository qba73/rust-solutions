use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}
