use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn pre_commit_hook_accepts_valid_commit() {
    let file = write_temp("feat: add feature");
    Command::new(cargo_bin!("pre-commit-hook"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn pre_commit_hook_rejects_invalid_commit() {
    let file = write_temp("invalid commit message");
    Command::new(cargo_bin!("pre-commit-hook"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn pre_commit_hook_handles_nonexistent_file() {
    Command::new(cargo_bin!("pre-commit-hook"))
        .arg("/nonexistent/path/to/commit/msg")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "failed to read commit message file",
        ));
}

#[test]
fn install_command_shows_help() {
    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install git commit-msg hook"));
}
