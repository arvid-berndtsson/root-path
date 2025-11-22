#[macro_use]
extern crate assert_cmd;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn cli_reads_from_stdin_valid_commit() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin
        .write_all(b"feat: add feature\n\nbody text")
        .expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        output.status.success(),
        "Valid commit from stdin should succeed"
    );
}

#[test]
fn cli_reads_from_stdin_invalid_commit() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin
        .write_all(b"invalid commit message")
        .expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        !output.status.success(),
        "Invalid commit from stdin should fail"
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");
    assert!(
        stderr.contains("Conventional commit check failed"),
        "Should show error message"
    );
}

#[test]
fn cli_reads_from_stdin_with_extra_types() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check", "--extra-types", "wip"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin
        .write_all(b"wip: work in progress")
        .expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        output.status.success(),
        "Commit with extra type from stdin should succeed"
    );
}

#[test]
fn cli_reads_from_stdin_empty_message() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin.write_all(b"").expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        !output.status.success(),
        "Empty commit from stdin should fail"
    );
    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");
    assert!(
        stderr.contains("empty commit message"),
        "Should show empty message error"
    );
}

#[test]
fn cli_reads_from_stdin_json_format() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check", "--format", "json"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin
        .write_all(b"feat: add feature")
        .expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        output.status.success(),
        "Valid commit from stdin should succeed"
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(
        stdout.contains("\"ok\":true"),
        "Should output JSON with ok:true"
    );
}

#[test]
fn cli_reads_from_stdin_json_format_error() {
    let mut cmd = Command::new(cargo_bin!("cc-check"));
    cmd.args(["check", "--format", "json"]);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("should spawn cc-check process");
    let mut stdin = child.stdin.take().expect("should get stdin handle");
    stdin
        .write_all(b"invalid commit")
        .expect("should write to stdin");
    drop(stdin);

    let output = child
        .wait_with_output()
        .expect("should wait for process output");
    assert!(
        !output.status.success(),
        "Invalid commit from stdin should fail"
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    assert!(
        stdout.contains("\"ok\":false"),
        "Should output JSON with ok:false"
    );
    assert!(
        stdout.contains("\"error\""),
        "Should include error field in JSON"
    );
}
