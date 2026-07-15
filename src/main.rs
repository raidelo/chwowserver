mod commands;
mod config;

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

/// Manage realmlist.wtf across one or more World of Warcraft installations.
#[derive(Parser)]
#[command(name = "chwowserver", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// New realmlist value to set (used when no subcommand is given), e.g. logon.warmane.com
    realmlist: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Register a WoW installation path
    AddPath {
        /// Absolute path to the WoW installation directory
        path: PathBuf,
    },
    /// List all registered WoW installation paths
    ListPaths,
    /// Remove a registered path by its index
    RmPath {
        /// Index of the path to remove (0-based)
        index: usize,
    },
    /// Show the realmlist.wtf content that was in place before the last change
    LastConfig,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::AddPath { path }) => commands::add_path::run(path),
        Some(Command::ListPaths) => commands::list_paths::run(),
        Some(Command::RmPath { index }) => commands::rm_path::run(index),
        Some(Command::LastConfig) => commands::last_config::run(),
        None => {
            let realmlist = cli
                .realmlist
                .ok_or_else(|| anyhow!("missing realmlist value. Usage: chwowserver <STRING>"))?;
            commands::set_realmlist::run(&realmlist)
        }
    }
}
