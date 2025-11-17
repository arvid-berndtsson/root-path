use cc_check::{first_meaningful_line, validate_header};

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
fn ignores_comments_for_first_line() {
    let msg = "# comment\n\n feat: ok";
    let first = first_meaningful_line(msg, true).unwrap();
    assert_eq!(first, "feat: ok");
}

#[test]
fn returns_none_for_empty_message() {
    let msg = "";
    assert!(first_meaningful_line(msg, true).is_none());
}

#[test]
fn returns_none_for_only_comments() {
    let msg = "# comment 1\n# comment 2";
    assert!(first_meaningful_line(msg, true).is_none());
}

#[test]
fn does_not_ignore_comments_when_disabled() {
    let msg = "# comment\n\nfeat: ok";
    let first = first_meaningful_line(msg, false);
    assert_eq!(first, Some("# comment".to_string()));
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
