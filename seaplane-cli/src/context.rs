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
use std::{
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, PoisonError},
};

use once_cell::sync::OnceCell;
use seaplane::api::{
    v1::formations::{Architecture, Flight as FlightModel, ImageReference},
    COMPUTE_API_URL,
};

use crate::{
    config::RawConfig,
    error::Result,
    fs,
    ops::flight::generate_name,
    printer::{ColorChoice, OutputFormat},
};

const FLIGHTS_FILE: &str = "flights.json";
const FORMATIONS_FILE: &str = "formations.json";

// The source of truth "Context" that is passed to all runtime processes to make decisions based
// on user configuration
pub struct Ctx {
    // @TODO we may need to get more granular than binary yes/no. For example, there may be times
    // when the answer is "yes...but only if the stream is a TTY." In these cases an enum of Never,
    // Auto, Always would be more appropriate
    //
    // Should be display ANSI color codes in output?
    pub color: ColorChoice,

    // The platform specific path to a data location
    data_dir: PathBuf,

    // How to display output
    pub out_format: OutputFormat,

    // Try to force the operation to happen
    pub force: bool,

    // Context relate to exclusively to Flight operations and commands
    pub flight: LateInit<FlightCtx>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            color: ColorChoice::Auto,
            data_dir: fs::data_dir(),
            out_format: OutputFormat::default(),
            force: false,
            flight: LateInit::default(),
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
            data_dir: fs::data_dir(),
            force: false,
            out_format: OutputFormat::default(),
            flight: LateInit::default(),
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

    #[inline]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

// Not deriving Clone or Debug on purpose due to cyclic OnceCells
pub struct FlightCtx {
    pub image: Option<ImageReference>,
    pub name: String,
    pub minimum: u64,
    pub maximum: Option<u64>,
    pub architecture: Vec<Architecture>,
    pub api_permission: bool,
    pub reset_maximum: bool,
}

impl Default for FlightCtx {
    fn default() -> Self {
        Self {
            name: generate_name(),
            image: None,
            minimum: 0,
            maximum: None,
            architecture: Vec::new(),
            api_permission: false,
            reset_maximum: false,
        }
    }
}

impl FlightCtx {
    /// Creates a new seaplane::api::v1::Flight from the contained values
    pub fn model(&self) -> FlightModel {
        // Create the new Flight model from the CLI inputs
        let mut flight_model = FlightModel::builder()
            .name(self.name.clone())
            .api_permission(self.api_permission)
            .minimum(self.minimum);

        if let Some(image) = self.image.clone() {
            flight_model = flight_model.image_reference(image);
        }

        // Due to Option<T> nature we conditionally a max
        if let Some(n) = self.maximum {
            flight_model = flight_model.maximum(n);
        }

        // Add all the architectures. In the CLI they're a Vec but in the Model they're a HashSet
        // which is the reason for the slightlly awkward loop
        for arch in &self.architecture {
            flight_model = flight_model.add_architecture(*arch);
        }

        // Create a new Flight struct we can add to our local JSON "DB"
        flight_model
            .build()
            .expect("Failed to build Flight from inputs")
    }
}

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
