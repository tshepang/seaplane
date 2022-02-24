//! Context describes the normalized and processed "settings" that a command can use at runtime.
//! This differs from the "config" or the "CLI Arguments" as the Context is built and updated from
//! those sources. This means the context is responsible for de-conflicting mutually exclusive
//! options, or overriding values.
//!
//! The Context is source of truth for all runtime decisions.
//!
//! The order of evaluation is as follows (note lower layers override layers above):
//!
//! 1. System configuration files are loaded (if any...currently none are defined)
//! 2. User configuration files are loaded (if any are found)
//! 3. Environment Variables (currently none are defined)
//! 4. Command Line Arguments
//!   4a. Because we use subcommands and global arguments each subcommand acts as it's own set of
//!   Command Line Arguments, and can thus affect the Context at each level in the command hierarchy.
//!   4b. Before the Context is handed off mutably to the next nested level, all updates from the
//!   parent should be finalized.
//!
//! After these steps the final Context is what is used to make runtime decisions.
use std::path::PathBuf;

use anyhow::Result;
use crate::{
    config::RawConfig,
    fs::data_dir,
    printer::{ColorChoice, OutputFormat},
};

const FLIGHTS_FILE: &str = "flights.json";
const FORMATIONS_FILE: &str = "formations.json";

// The source of truth "Context" that is passed to all runtime processes to make decisions based
// on user configuration
#[derive(Debug)]
pub struct Ctx {
    // Should be display ANSI color codes in output?
    pub color: ColorChoice,

    // The platform specific path to a data location
    pub data_dir: PathBuf,

    // How to display output
    pub out_format: OutputFormat,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            color: ColorChoice::Auto,
            data_dir: data_dir(),
            out_format: OutputFormat::default(),
        }
    }
}

impl Ctx {
    pub fn from_config(_cfg: &RawConfig) -> Result<Self> {
        // TODO: this just gets it compiling. Using `todo!` blocks progress since loading the
        // context happens at program startup, so we cannot panic on unimplemented

        Ok(Self {
            // We default to using color. Later when the context is updated from the CLI args, this
            // may change.
            color: ColorChoice::Auto,
            data_dir: data_dir(),
        })
    }

    pub fn update_from_env(&mut self) -> Result<()> {
        // TODO: this just gets it compiling. Using `todo!` blocks progress since loading the
        // context happens at program startup, so we cannot panic on unimplemented
        Ok(())
    }

    pub fn flights_file(&self) -> PathBuf {
        self.data_dir.join(FLIGHTS_FILE)
    }

    pub fn formations_file(&self) -> PathBuf {
        self.data_dir.join(FORMATIONS_FILE)
    }
}

}
