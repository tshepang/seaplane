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

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Result},
    fs::conf_dirs,
};

static SEAPLANE_CONFIG_FILE: &str = "seaplane.toml";

/// Extends a configuration instance with overriding config
pub trait ExtendConfig {
    fn extend(&mut self, other: &Self);
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "kebab-case")]
pub struct RawConfig {
    // Used to signal we already found a valid conifg and to warn the user we will be overriding
    #[serde(skip)]
    found: bool,

    #[serde(default)]
    pub account: RawAccountConfig,
}

impl RawConfig {
    pub fn load() -> Result<Self> {
        let mut cfg = RawConfig::default();

        for dir in conf_dirs() {
            let maybe_file = dir.join(SEAPLANE_CONFIG_FILE);

            cli_debugln!("Looking for configuration file at {:?}", maybe_file);
            if maybe_file.exists() {
                cli_debugln!("Found configuration file {:?}", maybe_file);
                cfg.update(maybe_file)?;
            }
        }

        Ok(cfg)
    }

    fn update<P: AsRef<Path>>(&mut self, p: P) -> Result<()> {
        if self.found {
            cli_warnln!(
                "overriding previous configuration options with {:?} (use --verbose for more info)",
                p.as_ref()
            );
        }

        let new_cfg: RawConfig = toml::from_str(&fs::read_to_string(p)?)?;

        // TODO: as we get more keys and tables we'll need a better way to do this
        if let Some(key) = new_cfg.account.api_key {
            self.account.api_key = Some(key);
        }

        self.found = true;
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "kebab-case")]
pub struct RawAccountConfig {
    #[serde(default)]
    pub api_key: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    

    #[test]
    fn deser_empty_config() {
        let cfg_str = r#"
        "#;

        let cfg: RawConfig = toml::from_str(cfg_str).unwrap();
        assert_eq!(cfg, RawConfig::default())
    }

    #[test]
    fn deser_empty_acount_config() {
        let cfg_str = r#"
        [account]
        "#;

        let cfg: RawConfig = toml::from_str(cfg_str).unwrap();
        assert_eq!(cfg, RawConfig::default())
    }

    #[test]
    fn deser_api_key() {
        let cfg_str = r#"
        [account]
        api-key = "abc123def456"
        "#;

        let cfg: RawConfig = toml::from_str(cfg_str).unwrap();

        assert_eq!(
            cfg,
            RawConfig {
                found: false,
                account: RawAccountConfig {
                    api_key: Some("abc123def456".into())
                }
            }
        )
    }
}
