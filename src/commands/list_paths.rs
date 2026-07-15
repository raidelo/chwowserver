use anyhow::Result;

use crate::config;

/// Prints every registered WoW installation path, prefixed by its index.
pub fn run() -> Result<()> {
    let cfg = config::load_config()?;

    if cfg.paths.is_empty() {
        println!("No paths registered yet. Use 'add-path <PATH>' to add one.");
        return Ok(());
    }

    for (index, path) in cfg.paths.iter().enumerate() {
        println!("{}: {}", index, path.display());
    }

    Ok(())
}
