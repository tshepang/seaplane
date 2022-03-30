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
pub mod metadata;
pub use metadata::MetadataCtx;

use std::{
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, PoisonError},
};

use clap_complete::Shell;
use once_cell::sync::OnceCell;

use crate::{
    config::RawConfig,
    error::{CliErrorKind, Context, Result},
    fs::{self, FromDisk, ToDisk},
    ops::{flight::Flights, formation::Formations},
    printer::{ColorChoice, OutputFormat},
};

const FLIGHTS_FILE: &str = "flights.json";
const FORMATIONS_FILE: &str = "formations.json";

#[derive(Debug, Default)]
pub struct Args {
    // @TODO we may need to get more granular than binary yes/no. For example, there may be times
    /// when the answer is "yes...but only if the stream is a TTY." In these cases an enum of Never,
    /// Auto, Always would be more appropriate
    ///
    /// Should be display ANSI color codes in output?
    pub color: ColorChoice,

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

    /// How to display output
    pub out_format: OutputFormat,

    /// Try to force the operation to happen
    pub force: bool,

    /// Do not use local state files
    pub stateless: bool,

    /// The API Key associated with an account provided by the CLI, env, or Config used to request
    /// access tokens
    pub api_key: Option<String>,
}

impl Args {
    pub fn api_key(&self) -> Result<&str> {
        self.api_key
            .as_deref()
            .ok_or_else(|| CliErrorKind::MissingApiKey.into_err())
    }
}

/// The source of truth "Context" that is passed to all runtime processes to make decisions based
/// on user configuration
// TODO: we may not want to derive this we implement circular references
#[derive(Debug)]
pub struct Ctx {
    /// The platform specific path to a data location
    data_dir: PathBuf,

    /// Context relate to exclusively to Flight operations and commands
    pub flight_ctx: LateInit<FlightCtx>,

    /// Context relate to exclusively to Formation operations and commands
    pub formation_ctx: LateInit<FormationCtx>,

    /// Context relate to exclusively to key-value operations and commands
    pub md_ctx: LateInit<MetadataCtx>,

    /// Where the configuration files were loaded from
    pub conf_files: Vec<PathBuf>,

    /// Common CLI arguments
    pub args: Args,

    /// The in memory databases
    pub db: Db,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            data_dir: fs::data_dir(),
            flight_ctx: LateInit::default(),
            formation_ctx: LateInit::default(),
            md_ctx: LateInit::default(),
            conf_files: Vec::new(),
            args: Args::default(),
            db: Db::default(),
        }
    }
}

impl From<RawConfig> for Ctx {
    fn from(cfg: RawConfig) -> Self {
        Self {
            data_dir: fs::data_dir(),
            conf_files: cfg.loaded_from.clone(),
            args: Args {
                // We default to using color. Later when the context is updated from the CLI args, this
                // may change.
                color: cfg.seaplane.color.unwrap_or_default(),
                api_key: cfg.account.api_key,
                ..Default::default()
            },
            ..Self::default()
        }
    }
}

impl Ctx {
    pub fn update_from_env(&mut self) -> Result<()> {
        // TODO: this just gets it compiling. Using `todo!` blocks progress since loading the
        // context happens at program startup, so we cannot panic on unimplemented
        Ok(())
    }

    #[inline]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn conf_files(&self) -> &[PathBuf] {
        &*self.conf_files
    }

    pub fn flights_file(&self) -> PathBuf {
        self.data_dir.join(FLIGHTS_FILE)
    }

    pub fn formations_file(&self) -> PathBuf {
        self.data_dir.join(FORMATIONS_FILE)
    }

    /// Write out an entirely new JSON file if `--stateless` wasn't used
    pub fn persist_formations(&self) -> Result<()> {
        self.db
            .formations
            .persist_if(!self.args.stateless)
            .with_context(|| format!("Path: {:?}\n", self.formations_file()))
    }

    /// Write out an entirely new JSON file if `--stateless` wasn't used
    pub fn persist_flights(&self) -> Result<()> {
        self.db
            .flights
            .persist_if(!self.args.stateless)
            .with_context(|| format!("Path: {:?}\n", self.flights_file()))
    }
}

/// The in memory "Databases"
#[derive(Debug, Default)]
pub struct Db {
    /// The in memory Flights database
    pub flights: Flights,

    /// The in memory Formations database
    pub formations: Formations,
}

impl Db {
    pub fn load<P: AsRef<Path>>(flights: P, formations: P) -> Result<Self> {
        Self::load_if(flights, formations, true)
    }

    pub fn load_if<P: AsRef<Path>>(flights: P, formations: P, yes: bool) -> Result<Self> {
        Ok(Self {
            flights: FromDisk::load_if(flights, yes).unwrap_or_else(|| Ok(Flights::default()))?,
            formations: FromDisk::load_if(formations, yes)
                .unwrap_or_else(|| Ok(Formations::default()))?,
        })
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
    pub fn get(&self) -> Option<&Mutex<T>> {
        self.inner.get()
    }
}

impl<T: Default> LateInit<T> {
    pub fn get_or_init(&self) -> MutexGuard<'_, T> {
        self.inner
            .get_or_init(|| Mutex::new(T::default()))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}
