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
//!
//! The context struct itself contains "global" values or those that apply in many scenarios or to
//! many commands. It also contains specialized contexts that contain values only relevant to those
//! commands or processes that need them. These specialized contexts should be lazily derived.

pub mod flight;
pub use flight::FlightCtx;
pub mod formation;
pub use formation::{FormationCfgCtx, FormationCtx};

use std::{
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, PoisonError},
};

use clap_complete::Shell;
use once_cell::sync::OnceCell;

use crate::{
    config::RawConfig,
    error::Result,
    fs,
    printer::{ColorChoice, OutputFormat},
};

const FLIGHTS_FILE: &str = "flights.json";
const FORMATIONS_FILE: &str = "formations.json";

/// The source of truth "Context" that is passed to all runtime processes to make decisions based
/// on user configuration
// TODO: we may not want to derive this we implement circular references
#[derive(Debug)]
pub struct Ctx {
    // @TODO we may need to get more granular than binary yes/no. For example, there may be times
    /// when the answer is "yes...but only if the stream is a TTY." In these cases an enum of Never,
    /// Auto, Always would be more appropriate
    ///
    /// Should be display ANSI color codes in output?
    pub color: ColorChoice,

    /// The platform specific path to a data location
    data_dir: PathBuf,

    /// How to display output
    pub out_format: OutputFormat,

    /// Try to force the operation to happen
    pub force: bool,

    /// The API Key associated with an account provided by the CLI, env, or Config used to request
    /// access tokens
    pub api_key: Option<String>,

    /// Context relate to exclusively to Flight operations and commands
    pub flight: LateInit<FlightCtx>,

    /// Context relate to exclusively to Formation operations and commands
    pub formation: LateInit<FormationCtx>,

    /// Where the configuration files were loaded from
    pub conf_files: Vec<PathBuf>,

    /// The name or local ID of an item
    pub name_id: Option<String>,

    /// What to overwrite
    pub overwrite: Option<String>,

    /// Do items need to be exact to match
    pub exact: bool,

    /// Match all items
    pub all: bool,

    /// Display third party licenses
    pub third_party: bool,

    /// The shell to generate completions for
    pub shell: Option<Shell>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            color: ColorChoice::Auto,
            data_dir: fs::data_dir(),
            out_format: OutputFormat::default(),
            force: false,
            api_key: None,
            flight: LateInit::default(),
            formation: LateInit::default(),
            conf_files: Vec::new(),
            overwrite: None,
            name_id: None,
            exact: false,
            all: false,
            third_party: false,
            shell: None,
        }
    }
}

impl Ctx {
    pub fn from_config(cfg: &RawConfig) -> Result<Self> {
        Ok(Self {
            // We default to using color. Later when the context is updated from the CLI args, this
            // may change.
            color: cfg.seaplane.color.unwrap_or_default(),
            data_dir: fs::data_dir(),
            api_key: cfg.account.api_key.clone(),
            conf_files: cfg.loaded_from.clone(),
            ..Self::default()
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

    pub fn flight_ctx(&self) -> MutexGuard<'_, FlightCtx> {
        self.flight
            .inner
            .get_or_init(|| Mutex::new(FlightCtx::default()))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn formation_ctx(&self) -> MutexGuard<'_, FormationCtx> {
        self.formation
            .inner
            .get_or_init(|| Mutex::new(FormationCtx::default()))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    #[inline]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn conf_files(&self) -> &[PathBuf] {
        &*self.conf_files
    }
}

// TODO: we may not want to derive this we implement circular references
#[derive(Debug)]
pub struct LateInit<T> {
    inner: OnceCell<Mutex<T>>,
}

impl<T> Default for LateInit<T> {
    fn default() -> Self {
        Self {
            inner: OnceCell::default(),
        }
    }
}

impl<T> LateInit<T> {
    pub fn init(&self, val: T) {
        assert!(self.inner.set(Mutex::new(val)).is_ok())
    }
}
