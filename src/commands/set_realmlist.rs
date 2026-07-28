use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config;

/// Iterates over every configured WoW installation, backs up the current
/// realmlist.wtf content, and overwrites it with the new realmlist value.
///
/// Errors for individual installations (missing file, permissions, etc.)
/// are reported to stderr but do not stop processing of the remaining
/// paths.
pub fn run(realmlist: &str) -> Result<()> {
    let cfg = config::load_config()?;

    if cfg.paths.is_empty() {
        println!("No paths registered. Use 'add-path <PATH>' to add a WoW installation first.");
        return Ok(());
    }

    let resolved_realmlist = cfg
        .mappings
        .as_ref()
        .and_then(|mappings| mappings.get(realmlist))
        .map(String::as_str)
        .unwrap_or(realmlist);

    if resolved_realmlist != realmlist {
        println!("Resolved mapping '{}' -> {}", realmlist, resolved_realmlist);
    }

    let mut backup = config::load_backup()?;
    let new_content = format!("set realmlist {}\n", resolved_realmlist);

    for install_path in &cfg.paths {
        let realmlist_file = config::realmlist_path(install_path);
        let key = install_path.display().to_string();

        match process_installation(&realmlist_file, &new_content) {
            Ok(previous_content) => {
                backup.entries.insert(key, previous_content);
                println!("OK: {}", realmlist_file.display());
            }
            Err(err) => {
                eprintln!("ERROR: {} -> {:#}", realmlist_file.display(), err);
            }
        }
    }

    config::save_backup(&backup)?;

    Ok(())
}

/// Reads the current content of `realmlist_file`, writes `new_content` in
/// its place, and returns the content that was there before the write.
fn process_installation(realmlist_file: &Path, new_content: &str) -> Result<String> {
    let previous_content = fs::read_to_string(realmlist_file)
        .with_context(|| format!("could not read {}", realmlist_file.display()))?;

    fs::write(realmlist_file, new_content)
        .with_context(|| format!("could not write {}", realmlist_file.display()))?;

    Ok(previous_content)
}
