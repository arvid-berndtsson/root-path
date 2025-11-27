use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Configuration for cc-check, loaded from .cc-check.toml
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Config {
    /// Additional commit types to allow (e.g., ["wip", "release"])
    pub extra_types: Vec<String>,
    
    /// Maximum subject length (0 to disable)
    pub max_subject: Option<usize>,
    
    /// Disallow trailing period in subject
    pub no_trailing_period: Option<bool>,
    
    /// Ignore comment lines (starting with '#') in commit message
    pub ignore_comments: Option<bool>,
    
    /// Allow merge-like messages (e.g., 'Merge ...' or 'Revert ...') to pass
    pub allow_merge_commits: Option<bool>,
}

impl Config {
    /// Load config from a TOML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read config file: {}", path.display()))?;
        
        toml::from_str(&contents)
            .with_context(|| format!("failed to parse config file: {}", path.display()))
    }
    
    /// Find and load config file from repository root
    /// Returns None if no config file is found
    pub fn load_from_repo() -> Result<Option<Self>> {
        let repo_root = crate::find_repo_root().ok();
        
        if let Some(root) = repo_root {
            let config_path = root.join(".cc-check.toml");
            if config_path.exists() {
                return Ok(Some(Self::from_file(&config_path)?));
            }
        }
        
        Ok(None)
    }
    
    /// Find the config file path in the repository
    pub fn find_config_file() -> Option<PathBuf> {
        let repo_root = crate::find_repo_root().ok()?;
        let config_path = repo_root.join(".cc-check.toml");
        
        if config_path.exists() {
            Some(config_path)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn parses_empty_config() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!(config.extra_types.len(), 0);
        assert_eq!(config.max_subject, None);
        assert_eq!(config.no_trailing_period, None);
    }
    
    #[test]
    fn parses_config_with_extra_types() {
        let toml = r#"
            extra_types = ["wip", "release"]
        "#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.extra_types, vec!["wip", "release"]);
    }
    
    #[test]
    fn parses_config_with_all_options() {
        let toml = r#"
            extra_types = ["wip", "release"]
            max_subject = 50
            no_trailing_period = false
            ignore_comments = false
            allow_merge_commits = false
        "#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.extra_types, vec!["wip", "release"]);
        assert_eq!(config.max_subject, Some(50));
        assert_eq!(config.no_trailing_period, Some(false));
        assert_eq!(config.ignore_comments, Some(false));
        assert_eq!(config.allow_merge_commits, Some(false));
    }
    
    #[test]
    fn loads_config_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".cc-check.toml");
        
        std::fs::write(&config_path, r#"
            extra_types = ["wip"]
            max_subject = 60
        "#).unwrap();
        
        let config = Config::from_file(&config_path).unwrap();
        assert_eq!(config.extra_types, vec!["wip"]);
        assert_eq!(config.max_subject, Some(60));
    }
    
    #[test]
    fn returns_error_for_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".cc-check.toml");
        
        std::fs::write(&config_path, "invalid toml {[}").unwrap();
        
        let result = Config::from_file(&config_path);
        assert!(result.is_err());
    }
}
