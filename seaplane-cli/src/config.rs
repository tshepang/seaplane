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

use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    error::{CliError, CliErrorKind, Result},
    fs::{conf_dirs, FromDisk, ToDisk},
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
    #[serde(skip)]
    pub loaded_from: Vec<PathBuf>,

    // Used to signal we already found a valid config and to warn the user we will be overriding
    #[serde(skip)]
    found: bool,

    #[serde(default)]
    pub account: RawAccountConfig,
}

impl RawConfig {
    /// Loads the Raw configuration file (not de-conflicted with the CLI or ENV yet)
    ///
    /// Loads configs from all platform specific locations, overriding values at each step
    pub fn load_all() -> Result<Self> {
        let mut cfg = RawConfig::default();

        for dir in conf_dirs() {
            let maybe_file = dir.join(SEAPLANE_CONFIG_FILE);

            let new_cfg = match RawConfig::load(&maybe_file) {
                Ok(cfg) => cfg,
                Err(e) => {
                    if e.kind() == &CliErrorKind::MissingPath {
                        continue;
                    }
                    return Err(e);
                }
            };

            if cfg.found {
                cli_warn!(@Yellow, "warn: ");
                cli_warnln!(@noprefix,
                    "overriding previous configuration options with {:?}",
                    maybe_file
                );
                cli_warn!("(hint: use ");
                cli_warn!(@Green, "--verbose ");
                cli_warnln!(@noprefix, "for more info)");
            }

            cfg.update(new_cfg)?;
            cfg.found = true;
        }

        Ok(cfg)
    }

    fn update(&mut self, new_cfg: RawConfig) -> Result<()> {
        // TODO: as we get more keys and tables we'll need a better way to do this
        if let Some(key) = new_cfg.account.api_key {
            self.account.api_key = Some(key);
        }
        self.loaded_from.extend(new_cfg.loaded_from);
        Ok(())
    }
}

impl FromDisk for RawConfig {
    fn set_loaded_from<P: AsRef<Path>>(&mut self, p: P) {
        self.loaded_from.push(p.as_ref().into());
    }

    fn loaded_from(&self) -> Option<&Path> {
        self.loaded_from.get(0).map(|p| &**p)
    }

    fn load<P: AsRef<Path>>(p: P) -> Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let path = p.as_ref();

        cli_debugln!("Looking for configuration file at {:?}", path);
        if !path.exists() {
            return Err(CliErrorKind::MissingPath.into_err());
        }

        cli_debugln!("Found configuration file {:?}", path);
        let mut cfg: RawConfig = toml::from_str(&fs::read_to_string(&p)?)?;
        cfg.set_loaded_from(p);
        Ok(cfg)
    }
}

impl ToDisk for RawConfig {
    fn persist(&self) -> Result<()>
    where
        Self: Sized + Serialize,
    {
        if let Some(path) = self.loaded_from.get(0) {
            let toml_str = toml::to_string_pretty(self)?;

            // TODO: make atomic so that we don't lose or corrupt data
            // TODO: long term consider something like SQLite
            fs::write(path, toml_str).map_err(CliError::from)
        } else {
            Err(CliErrorKind::MissingPath.into_err())
        }
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
    fn deser_empty_account_config() {
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
                loaded_from: Vec::new(),
                account: RawAccountConfig {
                    api_key: Some("abc123def456".into())
                }
            }
        )
    }
}
