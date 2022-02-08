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
use anyhow::Result;

use crate::config::Config;

// The source of truth "Context" that is passed to all runtime processes to make decisions based
// on user configuration
pub struct Ctx {
    // @TODO we may need to get more granular than binary yes/no. For example, there may be times
    // when the answer is "yes...but only if the stream is a TTY." In these cases an enum of Never,
    // Auto, Always would be more appropriate
    //
    // Should be display ANSI color codes in output?
    pub color: bool,
}

impl Default for Ctx {
    fn default() -> Self {
        Self { color: true }
    }
}

impl Ctx {
    pub fn from_config(cfg: &Config) -> Result<Self> {
        //todo!("impl Ctx::from_config")
        Ok(Self::default())
    }

    pub fn update_from_env(&mut self) -> Result<()> {
        //todo!("impl Ctx::update_from_env")
        Ok(())
    }
}
