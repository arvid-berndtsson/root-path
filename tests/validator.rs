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
