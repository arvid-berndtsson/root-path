use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_accepts_feat() {
    let file = write_temp("feat: add new feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_fix() {
    let file = write_temp("fix: correct bug");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_chore() {
    let file = write_temp("chore: update dependencies");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_docs() {
    let file = write_temp("docs: update README");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_style() {
    let file = write_temp("style: format code");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_refactor() {
    let file = write_temp("refactor: restructure code");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_perf() {
    let file = write_temp("perf: improve performance");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_test() {
    let file = write_temp("test: add unit tests");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_build() {
    let file = write_temp("build: update build config");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_ci() {
    let file = write_temp("ci: update workflow");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_revert() {
    let file = write_temp("revert: revert previous change");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_type_with_scope() {
    let file = write_temp("feat(api): add endpoint");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_type_with_breaking_change() {
    let file = write_temp("feat!: breaking change");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_type_with_scope_and_breaking_change() {
    let file = write_temp("feat(api)!: breaking api change");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_complex_scope() {
    let file = write_temp("fix(api/v2): handle edge case");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_scope_with_hyphens() {
    let file = write_temp("feat(api-client): add retry logic");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_scope_with_underscores() {
    let file = write_temp("fix(api_client): correct bug");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_accepts_scope_with_numbers() {
    let file = write_temp("feat(api2): add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}
