use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// e.g. "user@login.hpc.tntech.edu"
    pub ssh_target: String,
    /// default remote workdir
    pub default_workdir: String,
    /// cluster hostname (set by `auth login`)
    #[serde(default)]
    pub host: Option<String>,
    /// cluster username (set by `auth login`; never a password)
    #[serde(default)]
    pub username: Option<String>,
    /// local private key path for key auth (set by `auth login`)
    #[serde(default)]
    pub key_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ssh_target: "user@login.hpc.tntech.edu".to_string(),
            default_workdir: "$HOME".to_string(),
            host: Some("login.hpc.tntech.edu".to_string()),
            username: None,
            key_path: None,
        }
    }
}

pub fn config_path() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("tntech-hpc").join("config.toml"))
}

pub fn load() -> Result<AppConfig> {
    let path = match config_path() {
        Some(p) => p,
        None => return Ok(AppConfig::default()),
    };
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&text).context("parsing config.toml")
}

pub fn init() -> Result<std::path::PathBuf> {
    let path = config_path().context("no config dir")?;
    if !path.exists() {
        return save(&AppConfig::default());
    }
    Ok(path)
}

/// Write config with owner-only permissions (it holds identity, never secrets).
pub fn save(cfg: &AppConfig) -> Result<std::path::PathBuf> {
    let path = config_path().context("no config dir")?;
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, toml::to_string_pretty(cfg)?)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(path)
}
