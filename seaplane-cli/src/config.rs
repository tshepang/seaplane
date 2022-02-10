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
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

static SEAPLANE_CONFIG_FILE: &str = "seaplane.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RawConfig {
    pub dev: Option<HashMap<String, String>>,
}

impl RawConfig {
    pub fn load() -> Result<Self> {
        let mut cfg = RawConfig::default();

        for dir in search_directories() {
            let maybe_file = dir.join(SEAPLANE_CONFIG_FILE);

            debug!("Looking for configuration file at {:?}", maybe_file);
            if maybe_file.exists() {
                debug!("Found configuration file {:?}", maybe_file);
                cfg.update(maybe_file)?;
            }
        }

        Ok(cfg)
    }

    fn update<P: AsRef<Path>>(&mut self, p: P) -> Result<()> {
        let mut new_cfg: RawConfig = toml::from_str(&fs::read_to_string(p)?)?;

        let mut map = self.dev.take().unwrap_or_default();

        if new_cfg.dev.is_some() {
            map.extend(new_cfg.dev.take().unwrap());
            self.dev = Some(map);
        }

        Ok(())
    }
}

fn search_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(proj_dirs) = directories::ProjectDirs::from("io", "Seaplane", "seaplane") {
        dirs.push(proj_dirs.config_dir().to_owned());
    }
    if let Some(base_dirs) = directories::BaseDirs::new() {
        // On Linux ProjectDirs already adds ~/.config/seaplane, but not on macOS or Windows
        if !cfg!(target_os = "linux") {
            dirs.push(base_dirs.home_dir().join(".config/seaplane"));
        }
        dirs.push(base_dirs.home_dir().join(".seaplane"));
    }
    dirs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deser_empty_config_dev() {
        let cfg_str = r#"
        [dev]
        "#;

        let cfg: RawConfig = toml::from_str(cfg_str).unwrap();
        assert_eq!(
            cfg,
            RawConfig {
                dev: Some(HashMap::new())
            }
        )
    }

    #[test]
    fn deser_empty_config() {
        let cfg_str = r#"
        "#;

        let cfg: RawConfig = toml::from_str(cfg_str).unwrap();
        assert_eq!(cfg, RawConfig::default())
    }
}
