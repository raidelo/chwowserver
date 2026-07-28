use std::io::{Write, stdin, stdout};

use anyhow::Result;

use crate::config;

/// Adds a key-realmlist mapping to the config file.
pub fn run(key: String, realmlist: String, force: bool) -> Result<()> {
    let mut cfg = config::load_config()?;

    let mut mappings = cfg.mappings.clone().unwrap_or_default();

    if !force && let Some(value) = mappings.get(&key) {
        println!("Mapping already registered with provided key: {}", key);
        println!("Current value: {}", value);

        print!("Do you want to override it? (y\\N) ");
        stdout().flush()?;

        let mut input = String::with_capacity(1);

        stdin().read_line(&mut input)?;

        let lowercased_input = input.to_lowercase();

        if lowercased_input.trim() == "n" || lowercased_input.trim().is_empty() {
            println!("Aborting");
            return Ok(());
        }
    }

    mappings.insert(key.clone(), realmlist.clone());

    cfg.mappings = Some(mappings);

    config::save_config(&cfg)?;

    println!("Added mapping: {} -> {}", key, realmlist);
    Ok(())
}
