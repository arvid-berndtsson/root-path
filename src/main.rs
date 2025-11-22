use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use cc_check::{
    find_repo_root, first_meaningful_line, is_merge_like_header, validate_header, ValidationError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err("supported formats: text, json".to_string()),
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "cc-check",
    about = "Validate commit messages against Conventional Commits",
    version,
    arg_required_else_help = false
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the commit message file (for backward compatibility)
    #[arg(value_name = "COMMIT_MSG_FILE", hide = true)]
    commit_msg_file: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Install git commit-msg hook
    Install {
        /// Skip building the binary (assume it's already built)
        #[arg(long)]
        no_build: bool,
    },
    /// Validate a commit message
    Check {
        /// Path to the commit message file (as provided to commit-msg hook)
        #[arg(value_name = "COMMIT_MSG_FILE", required = false)]
        commit_msg_file: Option<PathBuf>,

        /// Allow types in addition to the default list (comma-separated)
        #[arg(long, value_name = "TYPES")]
        extra_types: Option<String>,

        /// Enforce max subject length (0 to disable)
        #[arg(long, default_value_t = 72)]
        max_subject: usize,

        /// Disallow trailing period in subject
        #[arg(long, default_value_t = true)]
        no_trailing_period: bool,

        /// Ignore comment lines (starting with '#') in commit message
        #[arg(long, default_value_t = true)]
        ignore_comments: bool,

        /// Allow merge-like messages (e.g., 'Merge ...' or 'Revert ...') to pass
        #[arg(long, default_value_t = true)]
        allow_merge_commits: bool,

        /// Output format: text or json
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Serialize)]
struct JsonResult<'a> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'a str>,
}

/// Options for checking commit messages
struct CheckOptions {
    /// Path to the commit message file
    commit_msg_file: Option<PathBuf>,
    /// Additional commit types to allow (comma-separated)
    extra_types: Option<String>,
    /// Maximum subject length (0 to disable)
    max_subject: usize,
    /// Disallow trailing period in subject
    no_trailing_period: bool,
    /// Ignore comment lines (starting with '#') in commit message
    ignore_comments: bool,
    /// Allow merge-like messages (e.g., 'Merge ...' or 'Revert ...') to pass
    allow_merge_commits: bool,
    /// Output format: text or json
    format: OutputFormat,
}

impl CheckOptions {
    /// Create default options for backward compatibility
    fn default_with_file(commit_msg_file: Option<PathBuf>) -> Self {
        Self {
            commit_msg_file,
            extra_types: None,
            max_subject: 72,
            no_trailing_period: true,
            ignore_comments: true,
            allow_merge_commits: true,
            format: OutputFormat::Text,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { no_build }) => install_hook(no_build),
        Some(Commands::Check {
            commit_msg_file,
            extra_types,
            max_subject,
            no_trailing_period,
            ignore_comments,
            allow_merge_commits,
            format,
        }) => check_commit(CheckOptions {
            commit_msg_file,
            extra_types,
            max_subject,
            no_trailing_period,
            ignore_comments,
            allow_merge_commits,
            format,
        }),
        None => {
            // Default behavior: check commit message (backward compatibility)
            check_commit(CheckOptions::default_with_file(cli.commit_msg_file))
        }
    }
}

fn install_hook(no_build: bool) -> Result<()> {
    // Find git directory
    let git_dir_output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()
        .context("failed to run git rev-parse --git-dir. Are you in a git repository?")?;

    if !git_dir_output.status.success() {
        bail!("not in a git repository");
    }

    let git_dir = String::from_utf8(git_dir_output.stdout)?.trim().to_string();
    let git_dir = if PathBuf::from(&git_dir).is_absolute() {
        PathBuf::from(git_dir)
    } else {
        std::env::current_dir()?.join(git_dir)
    };

    let hooks_dir = git_dir.join("hooks");
    // On Windows, use .bat extension for native Git compatibility
    // On Unix, use no extension (shell script)
    let hook_filename = if cfg!(windows) {
        "commit-msg.bat"
    } else {
        "commit-msg"
    };
    let commit_msg_hook = hooks_dir.join(hook_filename);

    // Build binary if needed
    if !no_build {
        println!("Building cc-check...");
        let status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .context("failed to run cargo build --release")?;

        if !status.success() {
            bail!("cargo build --release failed");
        }
    }

    // Find the binary path
    let exe_name = if cfg!(windows) {
        "cc-check.exe"
    } else {
        "cc-check"
    };
    let binary_path = std::env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("could not determine binary directory"))?
        .join(exe_name);

    // If we're running from cargo run, try to find the built binary
    let binary_path = if !binary_path.exists() {
        let repo_root = find_repo_root()?;
        let release_bin = repo_root.join("target/release").join(exe_name);
        if release_bin.exists() {
            release_bin
        } else {
            binary_path
        }
    } else {
        binary_path
    };

    // Create hooks directory
    fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("failed to create hooks directory: {}", hooks_dir.display()))?;

    // Backup existing hook
    if commit_msg_hook.exists() {
        // Use with_extension to replace the extension (e.g., commit-msg.bat -> commit-msg.backup)
        // This works correctly for both Windows (.bat) and Unix (no extension) hooks
        let backup = commit_msg_hook.with_extension("backup");
        if !backup.exists() {
            fs::copy(&commit_msg_hook, &backup).with_context(|| {
                format!(
                    "failed to backup existing hook: {}",
                    commit_msg_hook.display()
                )
            })?;
            println!("Backed up existing commit-msg hook to {}", backup.display());
        }
    }

    // Create hook script (Windows uses .bat, Unix uses shell script)
    let hook_content = if cfg!(windows) {
        create_windows_hook(&binary_path)?
    } else {
        create_unix_hook(&binary_path)?
    };

    fs::write(&commit_msg_hook, hook_content)
        .with_context(|| format!("failed to write hook: {}", commit_msg_hook.display()))?;

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&commit_msg_hook)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&commit_msg_hook, perms)?;
    }

    println!("✓ Commit-msg hook installed successfully!");
    println!();
    println!(
        "The hook will now validate all commit messages against the conventional commit format."
    );
    println!();
    println!("To test it, try committing with:");
    println!("  git commit -m \"invalid commit\"        # Will fail");
    println!("  git commit -m \"test: valid commit\"    # Will pass");

    Ok(())
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn create_windows_hook(binary_path: &Path) -> Result<String> {
    let binary_str = binary_path.to_string_lossy();
    // Windows batch files: wrap path in quotes to handle spaces
    // Escape any existing quotes by doubling them (batch file syntax)
    let escaped_binary = binary_str.replace('"', "\"\"");
    Ok(format!(
        r#"@echo off
REM Commit-msg hook to validate conventional commit messages
REM Generated by cc-check install

"{}" check "%~1"
if errorlevel 1 (
    echo.
    echo Commit rejected. Please fix your commit message to follow the conventional commit format.
    exit /b 1
)
"#,
        escaped_binary
    ))
}

fn create_unix_hook(binary_path: &Path) -> Result<String> {
    let binary_str = binary_path.to_string_lossy().replace('\\', "/");
    let escaped_binary = shell_escape(&binary_str);
    Ok(format!(
        r#"#!/bin/sh
# Commit-msg hook to validate conventional commit messages
# Generated by cc-check install

exec {} check "$1"
"#,
        escaped_binary
    ))
}

fn check_commit(options: CheckOptions) -> Result<()> {
    let allowed_types_default = [
        "feat", "fix", "chore", "docs", "style", "refactor", "perf", "test", "build", "ci",
        "revert",
    ];

    let mut allowed_types = allowed_types_default.map(String::from).to_vec();
    if let Some(extra) = &options.extra_types {
        for t in extra.split(',') {
            let t = t.trim();
            if !t.is_empty() && !allowed_types.iter().any(|x| x == t) {
                allowed_types.push(t.to_string());
            }
        }
    }

    let message = if let Some(path) = &options.commit_msg_file {
        fs::read_to_string(path)
            .with_context(|| format!("failed to read commit message file: {}", path.display()))?
    } else if !io::stdin().is_terminal() {
        // Read from stdin if it's not a TTY (i.e., piped input)
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("failed to read from stdin")?;
        buffer
    } else if let Ok(contents) = fs::read_to_string(".git/COMMIT_EDITMSG") {
        contents
    } else {
        bail!("no commit message file provided");
    };

    let header_opt = first_meaningful_line(&message, options.ignore_comments);
    let header = match header_opt {
        Some(h) => h,
        None => return exit_with(options.format, Err(ValidationError::Empty)),
    };

    if options.allow_merge_commits && is_merge_like_header(&header) {
        return exit_with(options.format, Ok(()));
    }

    let res = validate_header(
        &header,
        &allowed_types,
        options.max_subject,
        options.no_trailing_period,
    );
    exit_with(options.format, res)
}

fn exit_with(format: OutputFormat, res: std::result::Result<(), ValidationError>) -> Result<()> {
    match (format, res) {
        (OutputFormat::Text, Ok(())) => Ok(()),
        (OutputFormat::Text, Err(err)) => {
            eprintln!("Conventional commit check failed: {err}");
            std::process::exit(1);
        }
        (OutputFormat::Json, Ok(())) => {
            println!(
                "{}",
                serde_json::to_string(&JsonResult {
                    ok: true,
                    error: None
                })?
            );
            Ok(())
        }
        (OutputFormat::Json, Err(err)) => {
            println!(
                "{}",
                serde_json::to_string(&JsonResult {
                    ok: false,
                    error: Some(&err.to_string())
                })?
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    mod shell_escaping {
        use super::*;

        #[test]
        fn handles_regular_paths() {
            let path = "/usr/bin/cc-check";
            let escaped = shell_escape(path);
            assert_eq!(escaped, "'/usr/bin/cc-check'");
        }

        #[test]
        fn handles_paths_with_spaces() {
            let path = "/path with spaces/cc-check";
            let escaped = shell_escape(path);
            assert_eq!(escaped, "'/path with spaces/cc-check'");
        }

        #[test]
        fn handles_paths_with_quotes() {
            let path = "/path'with'quotes/cc-check";
            let escaped = shell_escape(path);
            assert_eq!(escaped, "'/path'\\''with'\\''quotes/cc-check'");
        }
    }

    mod unix_hook {
        use super::*;

        #[test]
        fn escapes_paths() {
            let path = PathBuf::from("/usr/bin/cc-check");
            let hook = create_unix_hook(&path).expect("should create unix hook for valid path");
            assert!(hook.contains("exec '/usr/bin/cc-check'"));
            assert!(hook.contains("#!/bin/sh"));
            assert!(hook.contains("check \"$1\""));
        }

        #[test]
        fn converts_windows_paths() {
            let path = PathBuf::from("C:\\Program Files\\cc-check.exe");
            let hook = create_unix_hook(&path).expect("should create unix hook for windows path");
            assert!(hook.contains("C:/Program Files/cc-check.exe"));
        }

        #[test]
        fn handles_paths_with_spaces() {
            let path = PathBuf::from("/path with spaces/cc-check");
            let hook =
                create_unix_hook(&path).expect("should create unix hook for path with spaces");
            assert!(hook.contains("'/path with spaces/cc-check'"));
        }
    }

    mod windows_hook {
        use super::*;

        #[test]
        fn wraps_paths_in_quotes() {
            let path = PathBuf::from("C:\\Program Files\\cc-check.exe");
            let hook = create_windows_hook(&path)
                .expect("should create windows hook for path with spaces");
            assert!(hook.contains("\"C:\\Program Files\\cc-check.exe\""));
            assert!(hook.contains("@echo off"));
            assert!(hook.contains("check \"%~1\""));
        }

        #[test]
        fn escapes_quotes_in_paths() {
            let path = PathBuf::from("C:\\path\"with\"quotes\\cc-check.exe");
            let hook = create_windows_hook(&path)
                .expect("should create windows hook for path with quotes");
            assert!(hook.contains("\"C:\\path\"\"with\"\"quotes\\cc-check.exe\""));
        }

        #[test]
        fn handles_paths_with_spaces() {
            let path = PathBuf::from("C:\\My Programs\\cc-check.exe");
            let hook = create_windows_hook(&path)
                .expect("should create windows hook for path with spaces");
            assert!(hook.contains("\"C:\\My Programs\\cc-check.exe\""));
        }
    }
}
