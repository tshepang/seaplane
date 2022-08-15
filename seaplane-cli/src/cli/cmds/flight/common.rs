use clap::{value_parser, Arg, ArgMatches};
use seaplane::api::v1::Architecture as ArchitectureModel;

use crate::cli::validator::validate_flight_name;

static LONG_IMAGE: &str =
    "The container image registry reference that this Flight will use (See IMAGE SPEC below)

NOTE at this time the if the registry is omitted, such as `nginxdemos/hello:latest` a default
registry of `registry.hub.docker.com` will be used. This may change in the future, so it is
recommended to always specify a full image reference path.";

static LONG_NAME: &str =
    "A human readable name for the Flight (must be unique within any Formation it

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future.";

static LONG_ARCHITECTURE: &str = "The architectures this flight is capable of running on. No value means it will be auto detected from the image definition

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

// We have to go through this routine of re-implementing to get around Rust's rule about not being
// allowed to implement traits on types not defined in the local crate.
/// Supported Architectures
#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, clap::ValueEnum)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Architecture {
    Amd64,
    Arm64,
}

impl Architecture {
    pub fn into_model(&self) -> ArchitectureModel { self.into() }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<ArchitectureModel> for &'a Architecture {
    fn into(self) -> ArchitectureModel {
        use Architecture::*;
        match self {
            Arm64 => ArchitectureModel::ARM64,
            Amd64 => ArchitectureModel::AMD64,
        }
    }
}

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation plan` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneFlightCommonArgMatches<'a>(pub &'a ArgMatches);

pub fn args(image_required: bool) -> Vec<Arg<'static>> {
    #[cfg_attr(not(feature = "unstable"), allow(unused_mut))]
    let mut hide = true;
    let _ = hide;
    #[cfg(feature = "unstable")]
    {
        hide = false;
    }
    vec![
        // TODO: allow omitting of USER (TENANT) portion of image spec too...but this requires an API
        // call to determine the TENANT id (at least until the `seaplane account login` command is done)
        arg!(--image|img =["SPEC"])
            .required(image_required)
            .help("The container image registry reference that this Flight will use (See IMAGE SPEC below)")
            .long_help(LONG_IMAGE),
        arg!(--name -('n') =["STRING"])
            .validator(validate_flight_name)
            .help("A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned")
            .long_help(LONG_NAME),
        arg!(--minimum|min =["NUM"=>"1"])
            .help("The minimum number of container instances that should ever be running"),
        arg!(--maximum|max =["NUM"])
            .overrides_with("no-maximum")
            .help("The maximum number of container instances that should ever be running (default: autoscale as needed)"),
        arg!(--architecture|arch|arches|architectures ignore_case =["ARCH"]...)
            .value_parser(value_parser!(Architecture))
            .help("The architectures this flight is capable of running on. No value means it will be auto detected from the image definition (supports comma separated list, or multiple uses)")
            .long_help(LONG_ARCHITECTURE),
        arg!(--("no-maximum")|("no-max"))
            .overrides_with("maximum")
            .help("There is no maximum number of instances"),
        arg!(--("api-permission")|("api-permissions"))
            .overrides_with("no-api-permission")
            .help("This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime")
            .hide(hide), // hidden on feature = unstable
        arg!(--("no-api-permission")|("no-api-permissions"))
            .overrides_with("api-permission")
            .help("This Flight should NOT be allowed to hit Seaplane API endpoints and will NOT be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime")
            .hide(hide), // hidden on feature = unstable
    ]
}
