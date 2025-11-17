use anyhow::{bail, Result};
use cc_check::find_repo_root;
use std::env;
use std::process::{Command, ExitStatus, Stdio};

fn get_exit_code(status: ExitStatus) -> i32 {
    // Process was terminated by a signal (Unix)
    // Use 130 (128 + SIGINT) as a reasonable default for signal termination
    status.code().unwrap_or(130)
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    // If first arg doesn't start with '-', it's the commit message file
    // We need to prepend "check" subcommand
    if !args.is_empty() && !args[0].starts_with('-') && args[0] != "check" {
        args.insert(0, "check".to_string());
    } else if args.is_empty() {
        args.push("check".to_string());
    }

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
        std::process::exit(get_exit_code(status));
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
        std::process::exit(get_exit_code(status));
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
        cmd.arg("--bin");
        cmd.arg("cc-check");
        cmd.arg("--");
        cmd.args(&args);
        cmd.current_dir(&repo_root);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        let status = cmd.status()?;
        std::process::exit(get_exit_code(status));
    }

    bail!(
        "cc-check binary not found and cargo is not available. \
         Please build with: cargo build --release"
    );
}
