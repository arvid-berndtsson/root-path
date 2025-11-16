use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn install_shows_help() {
    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install git commit-msg hook"));
}

#[test]
fn install_fails_outside_git_repo() {
    let temp_dir = TempDir::new().unwrap();
    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not in a git repository"));
}

#[test]
fn install_creates_hook_in_git_repo() {
    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let hooks_dir = git_dir.join("hooks");
    let hook_file = hooks_dir.join(if cfg!(windows) {
        "commit-msg.bat"
    } else {
        "commit-msg"
    });

    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    assert!(hook_file.exists(), "Hook file should be created");

    let hook_content = fs::read_to_string(&hook_file).unwrap();
    assert!(
        hook_content.contains("commit-msg") || hook_content.contains("Commit-msg"),
        "Hook should contain expected content"
    );
}

#[test]
fn install_backs_up_existing_hook() {
    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir).unwrap();

    let hook_file = hooks_dir.join(if cfg!(windows) {
        "commit-msg.bat"
    } else {
        "commit-msg"
    });
    let backup_file = hook_file.with_extension("backup");

    // Create existing hook
    fs::write(&hook_file, "#!/bin/sh\necho 'old hook'").unwrap();

    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    assert!(backup_file.exists(), "Backup file should be created");

    let backup_content = fs::read_to_string(&backup_file).unwrap();
    assert_eq!(
        backup_content, "#!/bin/sh\necho 'old hook'",
        "Backup should contain original content"
    );
}

#[test]
fn install_does_not_overwrite_backup() {
    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir).unwrap();

    let hook_file = hooks_dir.join(if cfg!(windows) {
        "commit-msg.bat"
    } else {
        "commit-msg"
    });
    let backup_file = hook_file.with_extension("backup");

    // Create existing hook and backup
    fs::write(&hook_file, "#!/bin/sh\necho 'old hook'").unwrap();
    fs::write(&backup_file, "#!/bin/sh\necho 'original backup'").unwrap();

    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Backup should not be overwritten
    let backup_content = fs::read_to_string(&backup_file).unwrap();
    assert_eq!(
        backup_content, "#!/bin/sh\necho 'original backup'",
        "Backup should not be overwritten"
    );
}

#[test]
fn install_hook_contains_binary_path() {
    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let hooks_dir = git_dir.join("hooks");
    let hook_file = hooks_dir.join(if cfg!(windows) {
        "commit-msg.bat"
    } else {
        "commit-msg"
    });

    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let hook_content = fs::read_to_string(&hook_file).unwrap();
    assert!(
        hook_content.contains("check"),
        "Hook should contain 'check' command"
    );
}

#[cfg(unix)]
#[test]
fn install_makes_hook_executable() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let hooks_dir = git_dir.join("hooks");
    let hook_file = hooks_dir.join("commit-msg");

    Command::new(cargo_bin!("cc-check"))
        .args(["install", "--no-build"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let metadata = fs::metadata(&hook_file).unwrap();
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    assert!(mode & 0o111 != 0, "Hook should be executable");
}
