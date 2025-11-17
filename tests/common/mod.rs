use std::io::Write;
use tempfile::NamedTempFile;

/// Helper function to create a temporary file with content for testing
pub fn write_temp(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("create temp file");
    write!(file, "{}", contents).expect("write temp file contents");
    file
}
