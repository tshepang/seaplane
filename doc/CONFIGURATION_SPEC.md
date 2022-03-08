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

### The `[seaplane]` Section

The first section in a `seaplane.toml` is the `[seaplane]` table which contains
various settings that apply to your local invocations of the `seaplane` CLI or
interactions with the APIs.

#### The `color` Field

Controls whether or not the output contains color. The field is a string and may contain one of the following values:

- `always`: The output will be colored even when STDOUT or STDERR is not a TTY
- `auto` (default): The output will be colored only if STDOUT/STDERR is a TTY and the `NO_COLOR` environment variable is not set
- `ansi`: If output is selected, always use ANSI color codes even on Windows (instead of console API calls)
- `never`: Do not color output

### The `[account]` Section

The second section in a `seaplane.toml` is the `[account]` table which contains your account
information for accessing Seaplane resources and APIs.

#### The `api-key` Field

Your API key is used to authenticate yourself to the Seaplane Authentication APIs which in turn
send back a short lived Authentication Token which authorizes actions against resources you own
such as your Formations and Flights.

The `api-key` field is a string.

[//]: # (links)

[toml]: https://toml.io/
