use anyhow::{bail, Result};
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Find the repository root by looking for Cargo.toml or .git directory
fn find_repo_root() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let mut dir = current_dir.as_path();

    loop {
        if dir.join("Cargo.toml").exists() || dir.join(".git").exists() {
            return Ok(dir.to_path_buf());
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => bail!("could not find repository root (no Cargo.toml or .git found)"),
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let repo_root = find_repo_root()?;

    // Try release binary first
    let release_bin = repo_root.join("target/release/cc-check");
    if release_bin.exists() && release_bin.is_file() {
        let mut cmd = Command::new(&release_bin);
        cmd.args(&args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let status = cmd.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }

    // Try debug binary
    let debug_bin = repo_root.join("target/debug/cc-check");
    if debug_bin.exists() && debug_bin.is_file() {
        let mut cmd = Command::new(&debug_bin);
        cmd.args(&args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let status = cmd.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }

    // Fall back to cargo run
    if Command::new("cargo")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        let mut cmd = Command::new("cargo");
        cmd.arg("run");
        cmd.arg("--quiet");
        cmd.arg("--release");
        cmd.arg("--");
        cmd.args(&args);
        cmd.current_dir(&repo_root);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let status = cmd.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }

    bail!(
        "cc-check binary not found and cargo is not available. \
         Please build with: cargo build --release"
    );
}
