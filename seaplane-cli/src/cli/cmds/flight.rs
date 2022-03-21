pub mod common;
mod copy;
mod create;
mod delete;
mod edit;
mod list;
#[cfg(feature = "unstable")]
mod template;

use clap::{ArgMatches, Command};
use seaplane::api::{
    v1::{ImageReference, ImageReferenceError},
    IMAGE_REGISTRY_URL,
};

#[cfg(feature = "unstable")]
pub use self::template::SeaplaneFlightTemplate;
pub use self::{
    copy::SeaplaneFlightCopy, create::SeaplaneFlightCreate, delete::SeaplaneFlightDelete,
    edit::SeaplaneFlightEdit, list::SeaplaneFlightList,
};
use crate::{
    cli::{specs::IMAGE_SPEC, CliCommand},
    error::{CliError, Result},
};

pub const FLIGHT_MINIMUM_DEFAULT: u64 = 1;

/// Allows eliding `registry.seaplanet.io` but otherwise just proxies parsing to ImageReference
pub fn str_to_image_ref(image_str: &str) -> Result<ImageReference> {
    match image_str.parse::<ImageReference>() {
        Ok(ir) => Ok(ir),
        Err(ImageReferenceError::ErrDomainInvalidFormat(_)) => {
            let ir: ImageReference = format!("{}{}", IMAGE_REGISTRY_URL, image_str).parse()?;
            Ok(ir)
        }
        Err(e) => Err(CliError::from(e)),
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlight;

impl SeaplaneFlight {
    pub fn command() -> Command<'static> {
        #[cfg_attr(not(feature = "unstable"), allow(unused_mut))]
        let mut app = Command::new("flight")
            .about("Operate on Seaplane Flights (logical containers), which are the core component of Formations")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(SeaplaneFlightCreate::command())
            .subcommand(SeaplaneFlightCopy::command())
            .subcommand(SeaplaneFlightEdit::command())
            .subcommand(SeaplaneFlightDelete::command())
            .subcommand(SeaplaneFlightList::command());

        #[cfg(feature = "unstable")]
        {
            app = app.subcommand(SeaplaneFlightTemplate::subcommand());
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
            Some(("create", m)) => Some((Box::new(SeaplaneFlightCreate), m)),
            Some(("copy", m)) => Some((Box::new(SeaplaneFlightCopy), m)),
            Some(("edit", m)) => Some((Box::new(SeaplaneFlightEdit), m)),
            Some(("delete", m)) => Some((Box::new(SeaplaneFlightDelete), m)),
            Some(("list", m)) => Some((Box::new(SeaplaneFlightList), m)),
            #[cfg(feature = "unstable")]
            Some(("template", m)) => Some((Box::new(SeaplaneFlightTemplate), m)),
            _ => None,
        }
    }
}
