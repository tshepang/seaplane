//! Config handles loading of, and updating the Context from, a configuration file.
//!
//! The config will look in several pre-determined (platform specific) locations. If a valid
//! configuration file is found, it's values are loaded. Note that later layers may override values
//! from previous layers.
//!
//! - System configuration files (currently none are defined)
//! - User configuration files
//!   - Linux
//!     - `$XDG_CONFIG_HOME/seaplane/`
//!     - `$HOME/.config/seaplane/`
//!     - `$HOME/.seaplane/`
//!   - macOS
//!     - `$HOME/Library/ApplicationSupport/io.Seaplane.seaplane/`
//!     - `$HOME/.config/seaplane/`
//!     - `$HOME/.seaplane/`
//!   - Windows
//!     - `%RoamingAppData%/Seaplane/seaplane/config/`
//!     - `$HOME/.config/seaplane/`
//!     - `$HOME/.seaplane/`
//! - The CLI's `--config` flag
//!
//! Note the CLI also provides a `--no-override` flag that prevents later configuration files from
//! overriding previously discovered configuration layers. In this case the final layer "wins" and
//! all previous layers are ignored. i.e. using `--config` will cause only that CLI provided
//! configuration to be considered and not any of those in the filesystem.
//!
//! See also the CONFIGURATION_SPEC.md in this repository
use std::path::PathBuf;

use anyhow::Result;

#[derive(Default)]
pub struct Config;

impl Config {
    pub fn load() -> Result<Self> {
        let mut cfg = Config::default();

        for _dir in search_directories() {}

        Ok(Config)
    }
}

fn search_directories() -> Vec<PathBuf> {
    Vec::new()
}
