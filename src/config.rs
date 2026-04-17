use color_eyre::eyre::{self, WrapErr};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub worktrees_dir: Option<String>,
    pub repos_dirs: Option<Vec<String>>,
    pub run_fetch: Option<bool>,
}

impl Config {
    pub fn load() -> eyre::Result<Self> {
        let config_dir = crate::dirs::get_config_dir()?;
        let config_path = config_dir.join("config.toml");
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&config_path)
            .wrap_err_with(|| format!("Failed to read config file: {}", config_path.display()))?;
        toml::from_str(&contents).wrap_err("Failed to parse config file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn load_from(toml_str: &str) -> eyre::Result<Config> {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, toml_str).unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        toml::from_str(&contents).wrap_err("Failed to parse config file")
    }

    #[test]
    fn missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        assert!(!path.exists());
        let config = Config::default();
        assert!(config.worktrees_dir.is_none());
        assert!(config.repos_dirs.is_none());
        assert!(config.run_fetch.is_none());
    }

    #[test]
    fn partial_config_parses() {
        let config = load_from(r#"worktrees_dir = "/tmp/worktrees""#).unwrap();
        assert_eq!(config.worktrees_dir.as_deref(), Some("/tmp/worktrees"));
        assert!(config.repos_dirs.is_none());
        assert!(config.run_fetch.is_none());
    }

    #[test]
    fn full_config_parses() {
        let config = load_from(
            r#"
worktrees_dir = "/tmp/worktrees"
repos_dirs    = ["/home/user/src", "/home/user/work"]
run_fetch     = true
"#,
        )
        .unwrap();
        assert_eq!(config.worktrees_dir.as_deref(), Some("/tmp/worktrees"));
        assert_eq!(
            config.repos_dirs.as_deref(),
            Some(&["/home/user/src".to_string(), "/home/user/work".to_string()][..])
        );
        assert_eq!(config.run_fetch, Some(true));
    }

    #[test]
    fn invalid_toml_returns_error() {
        let result = load_from("not valid toml ][");
        assert!(result.is_err());
    }
}
