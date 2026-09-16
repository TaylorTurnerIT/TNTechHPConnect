use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// e.g. "user@hpc.tntech.edu"
    pub ssh_target: String,
    /// default remote workdir
    pub default_workdir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ssh_target: "user@hpc.tntech.edu".to_string(),
            default_workdir: "$HOME".to_string(),
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
    std::fs::create_dir_all(path.parent().unwrap())?;
    if !path.exists() {
        std::fs::write(&path, toml::to_string_pretty(&AppConfig::default())?)?;
    }
    Ok(path)
}
