use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_accepts_valid_commit_text() {
    let file = write_temp("feat: add thing\n\nbody");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_invalid_type_text() {
    let file = write_temp("update: stuff");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn cli_allows_merge_like_by_default() {
    let file = write_temp("Merge branch 'feature/x'");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_trailing_period_by_default() {
    let file = write_temp("feat: add x.");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_empty_commit_message() {
    let file = write_temp("");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty commit message"));
}

#[test]
fn cli_rejects_commit_with_only_comments() {
    let file = write_temp("# comment 1\n# comment 2\n");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_commit_with_comments_by_default() {
    let file = write_temp("# comment\n\nfeat: actual commit");
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(file.path())
        .assert()
        .success();
}
