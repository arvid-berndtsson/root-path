use anyhow::{bail, Result};
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("empty commit message")]
    Empty,
    #[error("first line (header) missing")]
    MissingHeader,
    #[error("header must match <type>(<scope>)?: <subject>")]
    BadHeader,
    #[error("type '{0}' is not allowed")]
    DisallowedType(String),
    #[error("subject must be non-empty")]
    EmptySubject,
    #[error("subject exceeds {0} characters ({1})")]
    SubjectTooLong(usize, usize),
    #[error("subject must not end with a period")]
    TrailingPeriod,
}

/// Extract the first meaningful line from a commit message, skipping comment lines and empties.
pub fn first_meaningful_line(message: &str, ignore_comments: bool) -> Option<String> {
    for line in message.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if ignore_comments && trimmed.starts_with('#') {
            continue;
        }
        return Some(trimmed.to_string());
    }
    None
}

/// Return true if the message is a merge commit header and should be optionally allowed.
pub fn is_merge_like_header(line: &str) -> bool {
    line.starts_with("Merge ") || line.starts_with("Revert ")
}

pub fn validate_header(
    header_line: &str,
    allowed_types: &[String],
    max_subject_len: usize,
    no_trailing_period: bool,
) -> Result<(), ValidationError> {
    let header_re =
        Regex::new(r"^(?P<type>[a-z]+)(?P<scope>\([^)]+\))?(?P<bang>!)?: (?P<subject>.+)$")
            .expect("valid regex");

    let captures = header_re
        .captures(header_line)
        .ok_or(ValidationError::BadHeader)?;
    let commit_type = captures.name("type").map(|m| m.as_str()).unwrap_or("");
    let subject = captures
        .name("subject")
        .map(|m| m.as_str())
        .unwrap_or("")
        .trim();

    if !allowed_types.iter().any(|t| t == commit_type) {
        return Err(ValidationError::DisallowedType(commit_type.to_string()));
    }

    if subject.is_empty() {
        return Err(ValidationError::EmptySubject);
    }

    if max_subject_len > 0 && subject.chars().count() > max_subject_len {
        return Err(ValidationError::SubjectTooLong(
            max_subject_len,
            subject.chars().count(),
        ));
    }

    if no_trailing_period && subject.ends_with('.') {
        return Err(ValidationError::TrailingPeriod);
    }

    Ok(())
}

/// Find the repository root by looking for Cargo.toml or .git directory
pub fn find_repo_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn allowed() -> Vec<String> {
        vec![
            "feat", "fix", "chore", "docs", "style", "refactor", "perf", "test", "build", "ci",
            "revert",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    mod validation {
        use super::*;

        #[test]
        fn valid_minimal() {
            let header = "feat: add x";
            assert!(validate_header(header, &allowed(), 72, true).is_ok());
        }

        #[test]
        fn invalid_type() {
            let header = "update: stuff";
            assert!(validate_header(header, &allowed(), 72, true).is_err());
        }

        #[test]
        fn validates_with_scope() {
            let header = "feat(api): add endpoint";
            assert!(validate_header(header, &allowed(), 72, true).is_ok());
        }

        #[test]
        fn validates_with_breaking_change_bang() {
            let header = "feat!: breaking change";
            assert!(validate_header(header, &allowed(), 72, true).is_ok());
        }

        #[test]
        fn validates_with_scope_and_bang() {
            let header = "feat(api)!: breaking api change";
            assert!(validate_header(header, &allowed(), 72, true).is_ok());
        }

        #[test]
        fn rejects_empty_subject() {
            let header = "feat: ";
            assert!(validate_header(header, &allowed(), 72, true).is_err());
        }

        #[test]
        fn rejects_missing_colon() {
            let header = "feat add x";
            assert!(validate_header(header, &allowed(), 72, true).is_err());
        }

        #[test]
        fn rejects_invalid_scope_format() {
            let header = "feat[api]: add endpoint";
            assert!(validate_header(header, &allowed(), 72, true).is_err());
        }

        #[test]
        fn parses_with_scope_and_bang() {
            let header = "feat(api)!: x";
            assert!(validate_header(header, &allowed(), 72, true).is_ok());
        }

        #[test]
        fn rejects_trailing_period_when_enforced() {
            let header = "feat: x.";
            let err = validate_header(header, &allowed(), 72, true).unwrap_err();
            assert!(matches!(err, ValidationError::TrailingPeriod));
        }

        #[test]
        fn allows_trailing_period_when_disabled() {
            let header = "feat: x.";
            assert!(validate_header(header, &allowed(), 72, false).is_ok());
        }

        #[test]
        fn enforces_subject_length() {
            let long_subject = "a".repeat(80);
            let header = format!("feat: {}", long_subject);
            let err = validate_header(&header, &allowed(), 72, true).unwrap_err();
            assert!(matches!(err, ValidationError::SubjectTooLong(72, 80)));
        }
    }

    mod first_line {
        use super::*;

        #[test]
        fn first_meaningful_line_skips_comments_and_blanks() {
            let msg = "\n# comment\n\n  feat: ok";
            assert_eq!(
                first_meaningful_line(msg, true).as_deref(),
                Some("feat: ok")
            );
        }
    }

    mod merge_detection {
        use super::*;

        #[test]
        fn merge_like_headers_detected() {
            assert!(is_merge_like_header("Merge branch 'x'"));
            assert!(is_merge_like_header("Revert y"));
            assert!(!is_merge_like_header("feat: x"));
        }
    }

    mod repo_root {
        use super::*;
        use std::path::PathBuf;

        struct DirGuard {
            original_dir: PathBuf,
        }

        impl Drop for DirGuard {
            fn drop(&mut self) {
                // Try to restore the original directory, but don't panic if it fails
                // (e.g., if the directory was deleted in CI)
                let _ = std::env::set_current_dir(&self.original_dir);
            }
        }

        #[test]
        fn find_repo_root_finds_cargo_toml() {
            use tempfile::TempDir;
            let original_dir = std::env::current_dir().unwrap();
            let _guard = DirGuard {
                original_dir: original_dir.clone(),
            };
            let temp_dir = TempDir::new().unwrap();
            let temp_path = std::fs::canonicalize(temp_dir.path()).unwrap();
            let cargo_toml = temp_path.join("Cargo.toml");
            std::fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

            std::env::set_current_dir(&temp_path).unwrap();
            // Verify we're in the right directory and the file exists
            let current = std::env::current_dir().unwrap();
            assert!(cargo_toml.exists(), "Cargo.toml should exist at {:?}", cargo_toml);
            assert!(current.join("Cargo.toml").exists(), "Cargo.toml should exist in current dir {:?}", current);
            
            let root = find_repo_root().unwrap();
            // Use canonicalize to handle symlink differences (e.g., /var vs /private/var on macOS)
            let expected = temp_path;
            let actual = std::fs::canonicalize(&root).unwrap();
            assert_eq!(actual, expected);
        }

        #[test]
        fn find_repo_root_finds_git_dir() {
            use tempfile::TempDir;
            let original_dir = std::env::current_dir().unwrap();
            let _guard = DirGuard {
                original_dir: original_dir.clone(),
            };
            let temp_dir = TempDir::new().unwrap();
            let git_dir = temp_dir.path().join(".git");
            std::fs::create_dir(&git_dir).unwrap();

            std::env::set_current_dir(temp_dir.path()).unwrap();
            let root = find_repo_root().unwrap();
            // Use canonicalize to handle symlink differences (e.g., /var vs /private/var on macOS)
            let expected = std::fs::canonicalize(temp_dir.path()).unwrap();
            let actual = std::fs::canonicalize(&root).unwrap();
            assert_eq!(actual, expected);
        }

        #[test]
        fn find_repo_root_finds_parent_with_cargo_toml() {
            use tempfile::TempDir;
            let original_dir = std::env::current_dir().unwrap();
            let _guard = DirGuard {
                original_dir: original_dir.clone(),
            };
            let temp_dir = TempDir::new().unwrap();
            let sub_dir = temp_dir.path().join("sub").join("dir");
            std::fs::create_dir_all(&sub_dir).unwrap();
            let cargo_toml = temp_dir.path().join("Cargo.toml");
            std::fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

            std::env::set_current_dir(&sub_dir).unwrap();
            let root = find_repo_root().unwrap();
            // Use canonicalize to handle symlink differences (e.g., /var vs /private/var on macOS)
            let expected = std::fs::canonicalize(temp_dir.path()).unwrap();
            let actual = std::fs::canonicalize(&root).unwrap();
            assert_eq!(actual, expected);
        }

        #[test]
        fn find_repo_root_fails_when_no_repo_found() {
            use tempfile::TempDir;
            let original_dir = std::env::current_dir().unwrap();
            let _guard = DirGuard {
                original_dir: original_dir.clone(),
            };
            let temp_dir = TempDir::new().unwrap();
            let sub_dir = temp_dir.path().join("sub").join("dir");
            std::fs::create_dir_all(&sub_dir).unwrap();

            std::env::set_current_dir(&sub_dir).unwrap();
            let result = find_repo_root();
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("could not find repository root"));
        }
    }
}
