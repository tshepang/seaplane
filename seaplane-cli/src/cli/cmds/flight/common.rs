use clap::Arg;
use seaplane::{api::v1::Architecture, rexports::strum::VariantNames};

use crate::cli::validator::validate_name;

static LONG_IMAGE: &str =
    "The container image registry reference that this Flight will use (See IMAGE SPEC below)

All image references using the 'registry.seaplanet.io' registry may omit the domain portions of the
image reference as it is implied. For example, 'registry.seaplanet.io/USER/myimage:latest' can be
supplied simply as 'USER/myimage:latest'

NOTE at this time the only registry supported is registry.seaplanet.io. In the future when other
registries are supported, you must specify the full registry domain and path if using those
alternate registries in order to properly reference your image.";

static LONG_NAME: &str =
    "A human readable name for the Flight (must be unique within any Formation it

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future.";

pub fn args(image_required: bool) -> Vec<Arg<'static>> {
    vec![
        // TODO: allow omitting of USER (TENANT) portion of image spec too...but this requires an API
        // call to determine the TENANT id (at least until the `seaplane account login` command is done)
        arg!(--image|img =["SPEC"])
            .required(image_required)
            .help("The container image registry reference that this Flight will use (See IMAGE SPEC below)")
            .long_help(LONG_IMAGE),
        arg!(--name -('n') =["STRING"])
            .validator(validate_name)
            .help("A human readable name for the Flight (must be unique within any Formation it is a part of) if omitted a pseudo random name will be assigned")
            .long_help(LONG_NAME),
        arg!(--minimum|min =["NUM"=>"1"])
            .help("The minimum number of container instances that should ever be running"),
        arg!(--maximum|max =["NUM"])
            .overrides_with("no-maximum")
            .help("The maximum number of container instances that should ever be running (default: infinite)"),
        arg!(--architecture|arch|arches|architectures ignore_case =["ARCH"]...)
            .possible_values(Architecture::VARIANTS)
            .help("The architectures this flight is capable of running on. No value means it will be auto detected from the image definition"),
        arg!(--("api-permission")|("api-permissions"))
            .overrides_with("no-api-permission")
            .help("This Flight should be allowed to hit Seaplane API endpoints and will be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime"),
        arg!(--("no-api-permission")|("no-api-permissions"))
            .overrides_with("api-permission")
            .help("This Flight should NOT be allowed to hit Seaplane API endpoints and will NOT be provided a 'SEAPLANE_API_TOKEN' environment variable at runtime"),
        arg!(--("no-maximum")|("no-max"))
            .overrides_with("maximum")
            .help("There is no maximum number of instances")
    ]
}
