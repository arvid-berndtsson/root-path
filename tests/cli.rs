use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn write_temp(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("create temp file");
    write!(file, "{}", contents).expect("write temp file contents");
    file
}

#[test]
fn cli_accepts_valid_commit_text() {
    let file = write_temp("feat: add thing\n\nbody");
    Command::cargo_bin("cc-check")
        .unwrap()
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_invalid_type_text() {
    let file = write_temp("update: stuff");
    Command::cargo_bin("cc-check")
        .unwrap()
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn cli_json_ok() {
    let file = write_temp("fix: correct bug");
    Command::cargo_bin("cc-check")
        .unwrap()
        .args(["--format", "json"])
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("{\"ok\":true"));
}

#[test]
fn cli_json_error_contains_message() {
    let file = write_temp("wip: tmp");
    Command::cargo_bin("cc-check")
        .unwrap()
        .args(["--format", "json"])
        .arg(file.path())
        .assert()
        .failure()
        .stdout(
            predicate::str::contains("\"ok\":false").and(predicate::str::contains("not allowed")),
        );
}

#[test]
fn cli_allows_merge_like_by_default() {
    let file = write_temp("Merge branch 'feature/x'");
    Command::cargo_bin("cc-check")
        .unwrap()
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_trailing_period_by_default() {
    let file = write_temp("feat: add x.");
    Command::cargo_bin("cc-check")
        .unwrap()
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn pre_commit_hook_accepts_valid_commit() {
    let file = write_temp("feat: add feature");
    Command::cargo_bin("pre-commit-hook")
        .unwrap()
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn pre_commit_hook_rejects_invalid_commit() {
    let file = write_temp("invalid commit message");
    Command::cargo_bin("pre-commit-hook")
        .unwrap()
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn pre_commit_hook_handles_nonexistent_file() {
    Command::cargo_bin("pre-commit-hook")
        .unwrap()
        .arg("/nonexistent/path/to/commit/msg")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "failed to read commit message file",
        ));
}
