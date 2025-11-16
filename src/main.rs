use anyhow::{bail, Context, Result};
use clap::Parser;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use cc_check::{first_meaningful_line, is_merge_like_header, validate_header, ValidationError};

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
    version
)]
struct Cli {
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
}

#[derive(Serialize)]
struct JsonResult<'a> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'a str>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let allowed_types_default = [
        "feat", "fix", "chore", "docs", "style", "refactor", "perf", "test", "build", "ci",
        "revert",
    ];

    let mut allowed_types = allowed_types_default.map(String::from).to_vec();
    if let Some(extra) = &cli.extra_types {
        for t in extra.split(',') {
            let t = t.trim();
            if !t.is_empty() && !allowed_types.iter().any(|x| x == t) {
                allowed_types.push(t.to_string());
            }
        }
    }

    let message = if let Some(path) = cli.commit_msg_file {
        fs::read_to_string(&path)
            .with_context(|| format!("failed to read commit message file: {}", path.display()))?
    } else if let Ok(contents) = fs::read_to_string(".git/COMMIT_EDITMSG") {
        contents
    } else {
        bail!("no commit message file provided");
    };

    let header_opt = first_meaningful_line(&message, cli.ignore_comments);
    let header = match header_opt {
        Some(h) => h,
        None => return exit_with(cli.format, Err(ValidationError::Empty)),
    };

    if cli.allow_merge_commits && is_merge_like_header(&header) {
        return exit_with(cli.format, Ok(()));
    }

    let res = validate_header(
        &header,
        &allowed_types,
        cli.max_subject,
        cli.no_trailing_period,
    );
    exit_with(cli.format, res)
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
