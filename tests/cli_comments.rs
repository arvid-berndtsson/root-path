use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_ignores_comments_by_default() {
    let file = write_temp("# comment line\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_ignores_multiple_comments() {
    let file = write_temp("# comment 1\n# comment 2\n# comment 3\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_ignores_comments_with_whitespace() {
    let file = write_temp("  # comment with spaces\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_ignores_comments_after_blank_lines() {
    let file = write_temp("\n\n# comment\n\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_comments_in_body() {
    let file = write_temp("feat: add feature\n\n# This is a comment in the body\nMore body text");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_treats_comments_as_content_when_disabled() {
    let file = write_temp("# This is treated as content\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--ignore-comments=false"])
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_comment_like_text_in_subject() {
    // Subject contains # but it's not at the start, so it's valid
    let file = write_temp("feat: add #hashtag support");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_comment_character_in_scope() {
    // # character is allowed in scope (only at start of line is treated as comment)
    let file = write_temp("feat(api#v2): add endpoint");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_only_comments_with_blank_lines() {
    let file = write_temp("# comment 1\n# comment 2\n\n");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_comments_followed_by_valid_commit() {
    let file = write_temp("# Fixes issue #123\n# See also PR #456\n\nfeat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}
