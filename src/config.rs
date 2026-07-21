use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HtbConfig {
    #[serde(default = "default_output")]
    pub output: String,

    #[serde(default)]
    pub vpn_server: Option<u32>,

    #[serde(default)]
    pub no_color: bool,
}

fn default_output() -> String {
    "table".into()
}

impl Default for HtbConfig {
    fn default() -> Self {
        Self {
            output: default_output(),
            vpn_server: None,
            no_color: false,
        }
    }
}

impl HtbConfig {
    pub fn load(path: Option<&Path>) -> anyhow::Result<Self> {
        let config_path = match path {
            Some(p) => p.to_path_buf(),
            None => config_dir()?.join("config.toml"),
        };

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }
}

pub fn config_dir() -> Result<PathBuf, crate::error::HtbError> {
    dirs::home_dir()
        .ok_or_else(|| crate::error::HtbError::Config("could not determine home directory".into()))
        .map(|d| d.join(".htb-cli"))
}

pub fn token_path() -> Result<PathBuf, crate::error::HtbError> {
    Ok(config_dir()?.join(".token"))
}

pub fn read_token() -> Result<String, crate::error::HtbError> {
    let path = token_path()?;
    if !path.exists() {
        return Err(crate::error::HtbError::NotAuthenticated);
    }
    let token = fs::read_to_string(&path)?.trim().to_string();
    if token.is_empty() {
        return Err(crate::error::HtbError::NotAuthenticated);
    }
    Ok(token)
}

pub fn save_token(token: &str) -> anyhow::Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;

    let path = token_path()?;

    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)?;
        f.write_all(token.as_bytes())?;
    }

    #[cfg(not(unix))]
    {
        fs::write(&path, token)?;
    }

    Ok(())
}

pub fn remove_token() -> anyhow::Result<()> {
    let path = token_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = HtbConfig::default();
        assert_eq!(config.output, "table");
        assert_eq!(config.vpn_server, None);
        assert!(!config.no_color);
    }

    #[test]
    fn toml_round_trip() {
        let config = HtbConfig {
            output: "json".into(),
            vpn_server: Some(1),
            no_color: true,
        };
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: HtbConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.output, "json");
        assert_eq!(deserialized.vpn_server, Some(1));
        assert!(deserialized.no_color);
    }

    #[test]
    fn partial_toml_uses_defaults() {
        let input = r#"output = "json""#;
        let config: HtbConfig = toml::from_str(input).unwrap();
        assert_eq!(config.output, "json");
        assert_eq!(config.vpn_server, None);
        assert!(!config.no_color);
    }

    #[test]
    fn empty_toml_uses_defaults() {
        let config: HtbConfig = toml::from_str("").unwrap();
        assert_eq!(config.output, "table");
    }

    #[test]
    fn token_round_trip() {
        let dir = std::env::temp_dir().join("htb-cli-test-token");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(".token");

        std::fs::write(&path, "test-token-123").unwrap();
        let read_back = std::fs::read_to_string(&path).unwrap();
        assert_eq!(read_back.trim(), "test-token-123");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
