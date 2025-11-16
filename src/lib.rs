use regex::Regex;

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

    #[test]
    fn parses_with_scope_and_bang() {
        let header = "feat(api)!: break things";
        assert!(validate_header(header, &allowed(), 72, true).is_ok());
    }

    #[test]
    fn rejects_trailing_period_when_enforced() {
        let header = "feat: add x.";
        let err = validate_header(header, &allowed(), 72, true).unwrap_err();
        assert!(matches!(err, ValidationError::TrailingPeriod));
    }

    #[test]
    fn allows_trailing_period_when_disabled() {
        let header = "feat: add x.";
        assert!(validate_header(header, &allowed(), 72, false).is_ok());
    }

    #[test]
    fn enforces_subject_length() {
        let long_subject = "a".repeat(80);
        let header = format!("feat: {}", long_subject);
        let err = validate_header(&header, &allowed(), 72, true).unwrap_err();
        assert!(matches!(err, ValidationError::SubjectTooLong(72, 80)));
    }

    #[test]
    fn merge_like_headers_detected() {
        assert!(is_merge_like_header("Merge branch 'x'"));
        assert!(is_merge_like_header("Revert y"));
        assert!(!is_merge_like_header("feat: x"));
    }

    #[test]
    fn first_meaningful_line_skips_comments_and_blanks() {
        let msg = "\n# comment\n\n  feat: ok";
        assert_eq!(
            first_meaningful_line(msg, true).as_deref(),
            Some("feat: ok")
        );
    }
}
