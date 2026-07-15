# chwowserver

CLI tool to quickly switch `realmlist.wtf` across one or more World of
Warcraft installations, with automatic backup of the previous value.

## Configuration

All persistent state lives under `~/.config/chwowserver/`:

- `config.toml` — list of registered WoW installation paths.
- `last_config.toml` — backup of the `realmlist.wtf` content that was in
  place right before the last change, keyed by installation path.

Both files are created automatically on first use.

## Usage

```sh
# Register a WoW installation (must be an absolute, existing path)
chwowserver add-path /home/user/games/wow-retail

# List registered installations
chwowserver list-paths
# 0: /home/user/games/wow-retail

# Remove a registered installation by index
chwowserver rm-path 0

# Show the realmlist.wtf content that was overwritten last time
chwowserver last-config

# Switch every registered installation to a new realmlist
chwowserver logon.warmane.com
```

Running `chwowserver <STRING>` with no subcommand backs up each
installation's current `realmlist.wtf`, then overwrites it with:

```
set realmlist <STRING>
```

Errors for individual installations (missing file, permissions, etc.) are
reported per-path and do not stop processing of the remaining ones.

## Build

```sh
cargo build --release
```
