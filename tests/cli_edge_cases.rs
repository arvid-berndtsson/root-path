use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::process::Command;

mod common;
use common::write_temp;

#[test]
fn cli_handles_multiple_blank_lines() {
    let file = write_temp("\n\n\nfeat: add feature\n\n\n");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_leading_whitespace() {
    let file = write_temp("  feat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_trailing_whitespace() {
    let file = write_temp("feat: add feature  ");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_unicode_characters() {
    let file = write_temp("feat: add 日本語 support");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_emoji_in_subject() {
    let file = write_temp("feat: add 🎉 emoji support");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_special_characters() {
    let file = write_temp("feat: add support for @mentions and #hashtags");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_long_body() {
    let body = "a".repeat(1000);
    let file = write_temp(&format!("feat: add feature\n\n{}", body));
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_multiline_body() {
    let file = write_temp("feat: add feature\n\nLine 1\nLine 2\nLine 3");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_type_without_colon() {
    let file = write_temp("feat add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conventional commit check failed"));
}

#[test]
fn cli_rejects_empty_type() {
    let file = write_temp(": add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_invalid_scope_missing_paren() {
    let file = write_temp("feat api: add endpoint");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_invalid_scope_unclosed_paren() {
    let file = write_temp("feat(api: add endpoint");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_empty_scope() {
    let file = write_temp("feat(): add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_uppercase_type() {
    let file = write_temp("FEAT: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_type_with_numbers() {
    let file = write_temp("feat2: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_type_with_hyphens() {
    let file = write_temp("feat-api: add feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_revert_commit() {
    let file = write_temp("Revert \"feat: add feature\"");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_merge_commit_with_branch() {
    let file = write_temp("Merge branch 'feature/new-feature'");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_handles_merge_commit_with_pr() {
    let file = write_temp("Merge pull request #123 from user/feature");
    Command::new(cargo_bin!("cc-check"))
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_merge_when_disabled() {
    let file = write_temp("Merge branch 'feature'");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--allow-merge-commits=false"])
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_rejects_revert_when_merge_disabled() {
    let file = write_temp("Revert \"feat: add feature\"");
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--allow-merge-commits=false"])
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_subject_at_exact_length_limit() {
    let subject = "a".repeat(72);
    let file = write_temp(&format!("feat: {}", subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "72"])
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn cli_rejects_subject_one_over_limit() {
    let subject = "a".repeat(73);
    let file = write_temp(&format!("feat: {}", subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "72"])
        .arg(file.path())
        .assert()
        .failure();
}

#[test]
fn cli_handles_unicode_subject_length() {
    // Each emoji is typically 2-4 bytes but counts as 1 character
    let subject = "🎉".repeat(36); // 36 emojis = 72 characters
    let file = write_temp(&format!("feat: {}", subject));
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "72"])
        .arg(file.path())
        .assert()
        .success();
}
