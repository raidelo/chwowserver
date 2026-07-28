use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const CONFIG_DIR_NAME: &str = "chwowserver";
const CONFIG_FILE_NAME: &str = "config.toml";
const BACKUP_FILE_NAME: &str = "last_config.toml";
const REALMLIST_FILE_NAME: &str = "realmlist.wtf";

/// Main persistent configuration: the list of registered WoW installations.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub paths: Vec<PathBuf>,
    pub mappings: Option<HashMap<String, String>>,
}

/// Backup of the realmlist.wtf content that was overwritten for each
/// installation, keyed by the installation's path (as a string, since
/// TOML table keys must be strings).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Backup {
    #[serde(flatten, default)]
    pub entries: HashMap<String, String>,
}

/// Returns `~/.config/chwowserver`.
pub fn config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("could not determine the home directory")?;
    Ok(home.join(".config").join(CONFIG_DIR_NAME))
}

pub fn config_file_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}

pub fn backup_file_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(BACKUP_FILE_NAME))
}

/// Path to `realmlist.wtf` inside a given WoW installation directory.
pub fn realmlist_path(install_path: &Path) -> PathBuf {
    install_path.join(REALMLIST_FILE_NAME)
}

/// Creates the config directory if it does not exist yet.
fn ensure_config_dir() -> Result<PathBuf> {
    let dir = config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create config directory at {}", dir.display()))?;
    }
    Ok(dir)
}

/// Loads `config.toml`, creating an empty one on first use.
pub fn load_config() -> Result<Config> {
    ensure_config_dir()?;
    let path = config_file_path()?;

    if !path.exists() {
        let cfg = Config::default();
        save_config(&cfg)?;
        return Ok(cfg);
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file at {}", path.display()))?;
    let cfg: Config = toml::from_str(&content)
        .with_context(|| format!("failed to parse config file at {}", path.display()))?;
    Ok(cfg)
}

/// Writes `config.toml` back to disk.
pub fn save_config(cfg: &Config) -> Result<()> {
    ensure_config_dir()?;
    let path = config_file_path()?;
    let content = toml::to_string_pretty(cfg).context("failed to serialize configuration")?;
    fs::write(&path, content)
        .with_context(|| format!("failed to write config file at {}", path.display()))?;
    Ok(())
}

/// Loads `last_config.toml`, returning an empty backup if it does not
/// exist yet (e.g. before the first realmlist change).
pub fn load_backup() -> Result<Backup> {
    ensure_config_dir()?;
    let path = backup_file_path()?;

    if !path.exists() {
        return Ok(Backup::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read backup file at {}", path.display()))?;
    let backup: Backup = toml::from_str(&content)
        .with_context(|| format!("failed to parse backup file at {}", path.display()))?;
    Ok(backup)
}

/// Writes `last_config.toml` back to disk.
pub fn save_backup(backup: &Backup) -> Result<()> {
    ensure_config_dir()?;
    let path = backup_file_path()?;
    let content = toml::to_string_pretty(backup).context("failed to serialize backup data")?;
    fs::write(&path, content)
        .with_context(|| format!("failed to write backup file at {}", path.display()))?;
    Ok(())
}
