use anyhow::Result;

use crate::config;

/// Prints the realmlist.wtf content that was in place, for every
/// installation, right before the last time it was overwritten.
pub fn run() -> Result<()> {
    let backup = config::load_backup()?;

    if backup.entries.is_empty() {
        println!("No previous realmlist.wtf content has been backed up yet.");
        return Ok(());
    }

    for (path, content) in backup.entries.iter() {
        println!("=== {} ===", path);
        println!("{}", content);
    }

    Ok(())
}
