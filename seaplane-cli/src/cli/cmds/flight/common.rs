use clap::Parser;
use seaplane::{api::v1::Architecture, rexports::strum::VariantNames};

use crate::{
    cli::{cmds::flight::str_to_image_ref, validator::validate_name},
    context::FlightCtx,
    error::Result,
    ops::generate_name,
};

#[derive(Parser)]
pub struct SeaplaneFlightCommonArgs {
    // TODO: allow omitting of USER (TENANT) portion of image spec too...but this requires a an API
    // call to determine the TENANT id (at least until the `seaplane account login` command is done)
    /// The container image registry reference that this Flight will use (See IMAGE SPEC below)
    #[clap(
        long,
        visible_alias = "img",
        value_name = "SPEC",
        long_help = "The container image registry reference that this Flight will use (See IMAGE SPEC below)

All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
supplied simply as 'USER/myimage:latest'

NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
registries are supported, you must specify the full registry domain and path if using those
alternate registries in order to properly reference your image."
    )]
    pub image: Option<String>, // we use a string because we allow elision of the domain

    /// A human readable name for the Flight (must be unique within any Formation it is a part of)
    /// if omitted a pseudo random name will be assigned
    #[clap(
        short,
        long,
        validator = validate_name,
        long_help = "A human readable name for the Flight (must be unique within any Formation it

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future."
    )]
    pub name: Option<String>,

    /// The minimum number of container instances that should ever be running
    #[clap(long, default_value = "1", visible_alias = "min")]
    pub minimum: u64,

    /// The maximum number of container instances that should ever be running (default: infinite)
    #[clap(long, visible_alias = "max", overrides_with = "no-maximum")]
    pub maximum: Option<u64>,

    /// The architectures this flight is capable of running on. No value means it will be auto
    /// detected from the image definition.
    #[clap(long, ignore_case = true, visible_aliases = &["arch", "arches", "architectures"], possible_values = Architecture::VARIANTS, value_delimiter = ',')]
    pub architecture: Vec<Architecture>,

    /// This Flight should be allowed to hit Seaplane API endpoints and will be provided a
    /// 'SEAPLANE_API_TOKEN' environment variable at runtime
    #[clap(long, overrides_with = "no-api-permission", alias = "api-permissions")]
    pub api_permission: bool,

    /// This Flight should NOT be allowed to hit Seaplane API endpoints and will NOT be provided a
    /// 'SEAPLANE_API_TOKEN' environment variable at runtime
    #[clap(long, overrides_with = "api-permission", alias = "no-api-permissions")]
    pub no_api_permission: bool,

    /// There is no maximum number of instances
    #[clap(long, visible_alias = "no-max", overrides_with = "maximum")]
    pub no_maximum: bool,
}

impl SeaplaneFlightCommonArgs {
    pub fn flight_ctx(&self) -> Result<FlightCtx> {
        let mut generated_name = false;
        // We generate a random name if one is not provided
        let name = if let Some(name) = &self.name {
            name.to_owned()
        } else {
            generated_name = true;
            generate_name()
        };

        let image = if let Some(img_str) = &self.image {
            Some(str_to_image_ref(img_str)?)
        } else {
            None
        };

        Ok(FlightCtx {
            image,
            name,
            minimum: self.minimum,
            maximum: self.maximum,
            architecture: self.architecture.clone(),
            // because of clap overrides we only have to check api_permissions
            api_permission: self.api_permission,
            reset_maximum: self.no_maximum,
            generated_name,
        })
    }
}
