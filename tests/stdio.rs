#![cfg(test)]
pub mod utils;
pub use utils::*;

use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn prints_formatted_code() {
    let unformatted = b"function hello () { return \"world\"; }";
    let formatted = format!(
        "{}\n{}\n{}\n",
        "function hello() {", "  return 'world'", "}",
    );

    let mut child = Command::new(EXE)
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn child process");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(unformatted)
        .expect("write unformatted code to stdin");

    let output = child
        .wait_with_output()
        .expect("wait child process with output");

    assert_eq!(
        (
            u8v_to_utf8(&output.stdout),
            u8v_to_utf8(&output.stderr),
            output.status.success(),
        ),
        (formatted.as_str(), "", true),
    );
}

#[test]
fn parse_failure() {
    let unformatted = b"this is not a valid code";

    let mut child = Command::new(EXE)
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn child process");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(unformatted)
        .expect("write unformatted code to stdin");

    let output = child
        .wait_with_output()
        .expect("wait child process with output");

    assert_eq!(
        (u8v_to_utf8(&output.stdout), output.status.success()),
        ("", false),
        "stdout and status",
    );

    let stderr = u8v_to_utf8(&output.stderr);
    assert!(
        stderr.starts_with("Error: \"Failed to parse STDIN:"),
        "stderr: {}",
        stderr,
    );
}
