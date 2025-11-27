use assert_cmd::prelude::*;
#[macro_use]
extern crate assert_cmd;
use std::process::Command;
use tempfile::TempDir;

mod common;
use common::write_temp;

/// Helper to create a temp directory with a config file and Cargo.toml (for repo detection)
fn create_temp_repo_with_config(config_content: &str) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".cc-check.toml");
    let cargo_toml_path = temp_dir.path().join("Cargo.toml");
    
    std::fs::write(&config_path, config_content).unwrap();
    // Create a minimal Cargo.toml to make it look like a repo root
    std::fs::write(&cargo_toml_path, "[package]\nname = \"test\"").unwrap();
    
    temp_dir
}

#[test]
fn config_file_allows_extra_types() {
    let temp_repo = create_temp_repo_with_config(r#"
        extra_types = ["wip", "release"]
    "#);
    
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, "wip: work in progress").unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .success();
}

#[test]
fn config_file_sets_max_subject() {
    let temp_repo = create_temp_repo_with_config(r#"
        max_subject = 20
    "#);
    
    let long_subject = "a".repeat(25);
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, format!("feat: {}", long_subject)).unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .failure();
}

#[test]
fn config_file_allows_trailing_period() {
    let temp_repo = create_temp_repo_with_config(r#"
        no_trailing_period = false
    "#);
    
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, "feat: add feature.").unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .success();
}

#[test]
fn config_file_disables_merge_commits() {
    let temp_repo = create_temp_repo_with_config(r#"
        allow_merge_commits = false
    "#);
    
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, "Merge branch 'feature/x'").unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .failure();
}

#[test]
fn cli_flags_override_config_file() {
    let temp_repo = create_temp_repo_with_config(r#"
        max_subject = 20
    "#);
    
    // This would fail with max_subject = 20, but CLI overrides it to 50
    let long_subject = "a".repeat(30);
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, format!("feat: {}", long_subject)).unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--max-subject", "50"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .success();
}

#[test]
fn cli_extra_types_override_config_file() {
    let temp_repo = create_temp_repo_with_config(r#"
        extra_types = ["wip"]
    "#);
    
    // 'release' is not in config, but we override with CLI
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, "release: v1.0.0").unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check", "--extra-types", "release"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .success();
}

#[test]
fn works_without_config_file() {
    // Test that the tool still works when there's no config file
    let file = write_temp("feat: add feature");
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(file.path())
        .assert()
        .success();
}

#[test]
fn config_file_with_multiple_options() {
    let temp_repo = create_temp_repo_with_config(r#"
        extra_types = ["wip", "release"]
        max_subject = 50
        no_trailing_period = false
        allow_merge_commits = false
    "#);
    
    // Test a custom type
    let msg_file = temp_repo.path().join("commit_msg.txt");
    std::fs::write(&msg_file, "wip: work in progress.").unwrap();
    
    Command::new(cargo_bin!("cc-check"))
        .args(["check"])
        .arg(&msg_file)
        .current_dir(temp_repo.path())
        .assert()
        .success();
}
