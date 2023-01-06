pub mod common;
mod copy;
mod delete;
mod edit;
mod list;
mod plan;
#[cfg(feature = "unstable")]
mod template;

use clap::{ArgMatches, Command};
use seaplane::rexports::container_image_ref::{ImageReference, ImageReferenceError};

#[cfg(feature = "unstable")]
pub use self::template::SeaplaneFlightTemplate;
pub use self::{
    common::SeaplaneFlightCommonArgMatches, copy::SeaplaneFlightCopy, delete::SeaplaneFlightDelete,
    edit::SeaplaneFlightEdit, list::SeaplaneFlightList, plan::SeaplaneFlightPlan,
};
use crate::{
    cli::{specs::IMAGE_SPEC, CliCommand},
    error::{CliError, Result},
};

pub const FLIGHT_MINIMUM_DEFAULT: u64 = 1;

/// Allows eliding `registry` but otherwise just proxies parsing to ImageReference
pub fn str_to_image_ref(registry: &str, image_str: &str) -> Result<ImageReference> {
    match image_str.parse::<ImageReference>() {
        Ok(ir) => Ok(ir),
        Err(ImageReferenceError::ErrDomainInvalidFormat(_)) => {
            let ir: ImageReference = format!("{registry}/{image_str}").parse()?;
            Ok(ir)
        }
        Err(e) => Err(CliError::from(e)),
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlight;

impl SeaplaneFlight {
    pub fn command() -> Command {
        #[cfg_attr(not(feature = "unstable"), allow(unused_mut))]
        let mut app = Command::new("flight")
            .about("Operate on local Flight Plans which define \"Flights\" (logical containers), and are then referenced by Formations")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SeaplaneFlightPlan::command())
            .subcommand(SeaplaneFlightCopy::command())
            .subcommand(SeaplaneFlightEdit::command())
            .subcommand(SeaplaneFlightDelete::command())
            .subcommand(SeaplaneFlightList::command());

        #[cfg(feature = "unstable")]
        {
            app = app.subcommand(SeaplaneFlightTemplate::command());
        }
        app
    }
}

impl CliCommand for SeaplaneFlight {
    fn next_subcmd<'a>(
        &self,
        matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        match &matches.subcommand() {
            Some(("copy", m)) => Some((Box::new(SeaplaneFlightCopy), m)),
            Some(("edit", m)) => Some((Box::new(SeaplaneFlightEdit), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneFlightDelete), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneFlightList), m)),
            Some(("plan", m)) => Some((Box::new(SeaplaneFlightPlan), m)),
            #[cfg(feature = "unstable")]
            Some(("template", m)) => Some((Box::new(SeaplaneFlightTemplate), m)),
            _ => None,
        }
    }
}
