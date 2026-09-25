//! The released binary, run as a user runs it: it prints the checked sum.

#![forbid(unsafe_code)]

use std::process::Command;

#[test]
fn the_binary_prints_the_checked_sum() {
    let output = Command::new(env!("CARGO_BIN_EXE_maestro-release-canary"))
        .output()
        .expect("example binary must run");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"42\n");
    assert!(output.stderr.is_empty());
}
