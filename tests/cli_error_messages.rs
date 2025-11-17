use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_shows_empty_message_error() {
    let file = write_temp("");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty commit message"));
}

#[test]
fn cli_shows_disallowed_type_error() {
    let file = write_temp("invalid: commit message");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not allowed"));
}

#[test]
fn cli_shows_trailing_period_error() {
    let file = write_temp("feat: add feature.");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("period"));
}

#[test]
fn cli_shows_subject_too_long_error() {
    let long_subject = "a".repeat(100);
    let file = write_temp(&format!("feat: {}", long_subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "50"])
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds"));
}

#[test]
fn cli_shows_bad_header_error() {
    let file = write_temp("not a valid commit");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn cli_shows_file_not_found_error() {
    Command::new(cargo_bin!("cc-check"))
        .arg("/nonexistent/path/to/file")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "failed to read commit message file",
        ));
}

#[test]
fn cli_shows_empty_subject_error() {
    let file = write_temp("feat: ");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn cli_json_shows_error_message() {
    let file = write_temp("invalid: commit");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"ok\":false"))
        .stdout(predicate::str::contains("not allowed"));
}

#[test]
fn cli_json_shows_success() {
    let file = write_temp("feat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\":true"));
}

#[test]
fn cli_json_error_has_no_ok_field_when_false() {
    let file = write_temp("invalid: commit");
    let output = Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"ok\":false"));
    assert!(stdout.contains("\"error\""));
}

#[test]
fn cli_json_success_has_no_error_field() {
    let file = write_temp("feat: add feature");
    let output = Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"ok\":true"));
    // Error field should be omitted when ok is true
    assert!(!stdout.contains("\"error\""));
}
