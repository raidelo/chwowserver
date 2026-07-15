use anyhow::{Result, bail};

use crate::config;

/// Removes the path at `index` from the configuration.
pub fn run(index: usize) -> Result<()> {
    let mut cfg = config::load_config()?;

    if index >= cfg.paths.len() {
        bail!(
            "index {} is out of range (valid indices: 0..{})",
            index,
            cfg.paths.len()
        );
    }

    let removed = cfg.paths.remove(index);
    config::save_config(&cfg)?;

    println!("Removed path {}: {}", index, removed.display());
    Ok(())
}
