use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::config;

/// Adds a new WoW installation path to the configuration, validating that
/// it is absolute and that it exists on disk.
pub fn run(path: PathBuf) -> Result<()> {
    if !path.is_absolute() {
        bail!("path must be absolute: {}", path.display());
    }

    if !path.exists() {
        bail!("path does not exist: {}", path.display());
    }

    let mut cfg = config::load_config()?;

    if cfg.paths.contains(&path) {
        println!("Path already registered: {}", path.display());
        return Ok(());
    }

    cfg.paths.push(path.clone());
    config::save_config(&cfg)?;

    println!("Added path: {}", path.display());
    Ok(())
}
