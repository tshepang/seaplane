mod common;
mod copy;
mod create;
mod delete;
mod edit;
mod list;
#[cfg(feature = "unstable")]
mod template;

use clap::{Parser, Subcommand};
use seaplane::api::{
    v1::{ImageReference, ImageReferenceError},
    IMAGE_REGISTRY_URL,
};

#[cfg(feature = "unstable")]
pub use self::template::SeaplaneFlightTemplateArgs;
pub use self::{
    common::SeaplaneFlightCommonArgs, copy::SeaplaneFlightCopyArgs,
    create::SeaplaneFlightCreateArgs, delete::SeaplaneFlightDeleteArgs,
    edit::SeaplaneFlightEditArgs, list::SeaplaneFlightListArgs,
};
use crate::{
    cli::specs::IMAGE_SPEC,
    context::Ctx,
    error::{CliError, Result},
};

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

/// Operate on Seaplane Flights (logical containers), which are the core component of Formations
#[derive(Parser)]
pub struct SeaplaneFlightArgs {
    #[clap(subcommand)]
    cmd: SeaplaneFlightCmds,
}

impl SeaplaneFlightArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        use SeaplaneFlightCmds::*;

        match &self.cmd {
            Create(args) => Ok(args.run(ctx).map(|_| ())?),
            Copy(args) => args.run(ctx),
            Edit(args) => args.run(ctx),
            Delete(args) => args.run(ctx),
            List(args) => args.run(ctx),
            #[cfg(feature = "unstable")]
            Template(args) => args.run(ctx),
        }
    }
}

#[derive(Subcommand)]
pub enum SeaplaneFlightCmds {
    Create(SeaplaneFlightCreateArgs),
    Copy(SeaplaneFlightCopyArgs),
    Edit(SeaplaneFlightEditArgs),
    Delete(SeaplaneFlightDeleteArgs),
    List(SeaplaneFlightListArgs),
    #[cfg(feature = "unstable")]
    Template(SeaplaneFlightTemplateArgs),
}
