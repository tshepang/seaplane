# Seaplane Configuration Specification

This document describes the `seaplane` configuration file and it's specification.

## Search Locations

On startup, `seaplane` will check in platform specific locations for a configuration file. If
found, the file will be loaded.

The following locations and order are used.

### Linux

- `$XDG_CONFIG_HOME/seaplane/`
- `$HOME/.config/seaplane/`
- `$HOME/.seaplane/`

### macOS

- `$HOME/Library/ApplicationSupport/io.Seaplane.seaplane/`
- `$HOME/.config/seaplane/`
- `$HOME/.seaplane/`

### Windows

- `%RoamingAppData%/Seaplane/seaplane/config/`
- `$HOME/.config/seaplane/`
- `$HOME/.seaplane/`

Alternatively, a custom file may be specified at the command line use the appropriate flags.

## Format

The `seaplane` configuration file is in [TOML][toml] format and consists of the following sections.

!!! Note
    Most sections may be omitted, and their default values will be used.

### The `[dev]` Section

The `[dev]` section defines values related to the local-development `seaplane dev` command.

[//]: # (links)

[toml]: https://toml.io/
