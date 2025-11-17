use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_accepts_extra_types() {
    let file = write_temp("wip: work in progress");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--extra-types", "wip"])
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_multiple_extra_types() {
    let file = write_temp("release: v1.0.0");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--extra-types", "wip,release"])
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_merge_commits_when_disabled() {
    let file = write_temp("Merge branch 'feature/x'");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--allow-merge-commits=false"])
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_disables_subject_length_check() {
    let long_subject = "a".repeat(100);
    let file = write_temp(&format!("feat: {}", long_subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "0"])
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_enforces_custom_subject_length() {
    let long_subject = "a".repeat(50);
    let file = write_temp(&format!("feat: {}", long_subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "40"])
        .arg(file.path())
        .assert()
        .failure();
}
