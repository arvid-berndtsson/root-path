use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_json_ok() {
    let file = write_temp("fix: correct bug");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("{\"ok\":true"));
}

#[test]
fn cli_json_error_contains_message() {
    let file = write_temp("wip: tmp");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--format", "json"])
        .arg(file.path())
        .assert()
        .failure()
        .stdout(
            predicate::str::contains("\"ok\":false").and(predicate::str::contains("not allowed")),
        );
}
