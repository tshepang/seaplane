//! The common args effectively represent a "Formation Configuration" which many commands can
//! include.
//!
//! The only additional information is the formation name, which is not part of the configuration,
//! but many commands need as well.

use clap::Arg;

use seaplane::api::v1::{Provider as ProviderModel, Region as RegionModel};
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::cli::validator::{
    validate_endpoint, validate_formation_name, validate_name_id, validate_name_id_path_inline,
    validate_public_endpoint,
};

static LONG_NAME: &str =
    "A human readable name for the Formation (must be unique within the tenant)

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future.";

static LONG_AFFINITY: &str = "A Formation Instance that this Formation has an affinity for.

This is a hint to the scheduler to place containers running in each of these
formations \"close\" to eachother (for some version of close including but
not limited to latency).

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_CONNECTION: &str = "A Formation Instance that this Formation is connected to.

Two formations can communicate over their formation endpoints (the endpoints configured via
--formation-endpoints) if and only if both formations opt in to that connection (list
each other in their connections map)

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_PUBLIC_ENDPOINT: &str = r#"An endpoint that will publicly exposed on Instances of this Formation Plan

Public Endpoints take the form '{ROUTE}={FLIGHT}:{PORT}'. Where

ROUTE  := An HTTP URL route
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes which Flight and port should serve the HTTP traffic arriving at this Formation's
domain URL using the specified route.

For example, consider:

$ seaplane formation edit Foo --public-endpoint /foo/bar=baz:1234

Would mean, all HTTP traffic from the public internet hitting the route '/foo/bar' on the 'Foo'
Formation's domain should be directed to this Formation's Flight named 'baz' on port '1234'

In the future, support for other protocols such as 'tcp:port' or 'udp:port' may be added alongside
'http' routes.

Multiple items can be passed as a comma separated list, or by using the argument
multiple times."#;

static LONG_FORMATION_ENDPOINT: &str = r#"An endpoint that will only exposed privately by Instances of this Formation Plan (only exposed to other Formations)

Formation Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where

PROTO  := [http | https] | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route, can be elided
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at instances of this Formation's domains URL from the private network
should be sent.

For example, consider:

$ seaplane formation edit Foo --formation-endpoint tcp:22=baz:2222

Would mean, route all traffic arriving to the 'Foo' Formation's domain URL on TCP/22 from the
private network to the the Formation's Flight named 'baz' on port '2222'. The PROTO of the incoming
traffic will be used for the PROTO of the outgoing traffic to FLIGHT

Note 'https' can be used interchangeably with 'http' for convenience sake. It does NOT however
require the traffic actually be HTTPS. Here 'http' (or convenience 'https') simply means "Traffic
using the HTTP" protocol.

Multiple items can be passed as a comma separated list, or by using the argument
multiple times."#;

static LONG_FLIGHT_ENDPOINT: &str = r#"An endpoint that will only be exposed privately on Instances of this Formation Plan (only exposed to Flights within this same Formation Instance)

Flight Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where

PROTO  := [http | https] | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route, can be elided
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this Formation's domain URL from within this Formation's
private network should be sent.

For example, consider:

$ seaplane formation edit Foo --flight-endpoint udp:1234=baz:4321

Would mean, route all traffic arriving to the 'Foo' Formation's domain URL on UDP/1234 from the
Formation's private network to the the Formation's Flight named 'baz' on port '4321'. The PROTO of
the incoming traffic will be used for the PROTO of the outgoing traffic to FLIGHT

Note 'https' can be used interchangeably with 'http' for convenience sake. It does NOT however
require the traffic actually be HTTPS. Here 'http' (or convenience 'https') simply means "Traffic
using the HTTP" protocol.

Multiple items can be passed as a comma separated list, or by using the argument
multiple times."#;

static LONG_REGION: &str =
    "A region in which this Formation's Flights are allowed to run in (See REGION SPEC below)

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_PROVIDER: &str = "A provider that this Formation's Flights are permitted to run on

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_EXCLUDE_PROVIDER: &str =
    "A provider that this Formation's Flights are *NOT* permitted to run on

This will override any values given to --provider

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_EXCLUDE_REGION: &str =
    "A region in which this Formation's Flights are *NOT* allowed to run in (See REGION SPEC below)

This will override any values given to --region

Multiple items can be passed as a comma separated list, or by using the argument
multiple times.";

static LONG_FLIGHT: &str =
    "A Flight Plan to include in this Formation in the form of ID|NAME|@path|@-|INLINE-SPEC (See FLIGHT SPEC below)

Multiple items can be passed as a SEMICOLON (';') separated list or by using the argument multiple
times. Note that when using the INLINE-SPEC it's usually easiest to only place one Flight Plan per
--include-flight-plan argument

$ seaplane formation plan \\
    --include-flight-plan name=flight1,image=nginx:latest \\
    --include-flight-plan name=flight2,image=hello:latest

Which would create, and include, two Flight Plans (flight1, and flight2).";
/// We provide a shim between the Seaplane Provider so we can do some additional UX work like 'all'
#[derive(Debug, Copy, Clone, PartialEq, EnumString, EnumVariantNames)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Provider {
    Aws,
    Azure,
    DigitalOcean,
    Equinix,
    Gcp,
    All,
}

impl Provider {
    pub fn into_model(&self) -> Option<ProviderModel> {
        if self == &Provider::All {
            None
        } else {
            Some(self.into())
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<ProviderModel> for &'a Provider {
    fn into(self) -> ProviderModel {
        use Provider::*;
        match self {
            Aws => ProviderModel::AWS,
            Azure => ProviderModel::Azure,
            DigitalOcean => ProviderModel::DigitalOcean,
            Equinix => ProviderModel::Equinix,
            Gcp => ProviderModel::GCP,
            All => panic!("Provider::All cannot be converted into seaplane::api::v1::Provider"),
        }
    }
}

// TODO: @needs-decision @pre-1.0 we need to come back and address how many aliases there are and
// their sort order for the CLI's "possible values" message. They're currently "grouped" in region
// aliases, but for one it's too many values, also it makes the sort look wild in the help message.
// The Compute API only uses the XA, XC, XE values, but those are the least user friendly.
/// We provide a shim between the Seaplane Region so we can do some additional UX work
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, EnumString, EnumVariantNames)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Region {
    XA,
    Asia,
    XC,
    PRC,
    PeoplesRepublicofChina,
    XE,
    Europe,
    EU,
    XF,
    Africa,
    XN,
    NorthAmerica,
    NAmerica,
    XO,
    Oceania,
    XQ,
    Antarctica,
    XS,
    SAmerica,
    SouthAmerica,
    XU,
    UK,
    UnitedKingdom,
    All,
}

impl Region {
    pub fn into_model(&self) -> Option<RegionModel> {
        if self == &Region::All {
            None
        } else {
            Some(self.into())
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<RegionModel> for &'a Region {
    fn into(self) -> RegionModel {
        use Region::*;
        match self {
            XA | Asia => RegionModel::XA,
            XC | PRC | PeoplesRepublicofChina => RegionModel::XC,
            XE | Europe | EU => RegionModel::XE,
            XF | Africa => RegionModel::XF,
            XN | NorthAmerica | NAmerica => RegionModel::XN,
            XO | Oceania => RegionModel::XO,
            XQ | Antarctica => RegionModel::XQ,
            XS | SAmerica | SouthAmerica => RegionModel::XS,
            XU | UK | UnitedKingdom => RegionModel::XU,
            All => panic!("Region::All cannot be converted into seaplane::api::v1::Region"),
        }
    }
}

pub fn args() -> Vec<Arg<'static>> {
    let validator = |s: &str| validate_name_id(validate_formation_name, s);
    #[cfg_attr(not(feature = "unstable"), allow(unused_mut))]
    let mut hide = true;
    let _ = hide;
    #[cfg(feature = "unstable")]
    {
        hide = false;
    }

    // TODO: add --from with support for @file and @- (stdin)
    vec![
        arg!(name_id --name -('n') =["STRING"])
            .help("A human readable name for the Formation (must be unique within the tenant) if omitted a pseudo random name will be assigned")
            .long_help(LONG_NAME)
            .validator(validate_formation_name),
        arg!(--launch|active)
            .overrides_with("launch") // Override with self so someone can do `--launch --active` which isn't needed, but people will do it
            .help("This Formation Plan should be deployed and set as active right away (requires a formation configuration)"),
        arg!(--grounded|("no-active"))
            .help("This Formation Plan should be deployed but NOT set as active (requires a formation configuration)"),
        arg!(--("include-flight-plan")|("include-flight-plans") -('I') =["SPEC"]...)
            .help("Use local Flight Plan in this Formation in the form of ID|NAME|@path|@-|INLINE-SPEC (supports SEMICOLON (';') separated list, or multiple uses) (See FLIGHT SPEC below)")
            .value_delimiter(';')
            .long_help(LONG_FLIGHT)
            .validator(validate_name_id_path_inline),
        arg!(--provider|providers =["PROVIDER"=>"all"]... ignore_case)
            .help("A provider that this Formation's Flights are permitted to run on (supports comma separated list, or multiple uses)")
            .long_help(LONG_PROVIDER)
            .possible_values(Provider::VARIANTS),
        arg!(--("exclude-provider")|("exclude-providers") =["PROVIDER"]... ignore_case)
            .help("A provider that this Formation's Flights are *NOT* permitted to run on (supports comma separated list, or multiple uses)")
            .long_help(LONG_EXCLUDE_PROVIDER)
            .possible_values(Provider::VARIANTS),
        arg!(--region|regions =["REGION"=>"all"]... ignore_case)
            .help("A region in which this Formation's Flights are allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below)")
            .long_help(LONG_REGION)
            .possible_values(Region::VARIANTS),
        arg!(--("exclude-region")|("exclude-regions") =["REGION"]... ignore_case)
            .help("A region in which this Formation's Flights are *NOT* allowed to run in (supports comma separated list, or multiple uses) (See REGION SPEC below)")
            .long_help(LONG_EXCLUDE_REGION)
            .possible_values(Region::VARIANTS),
        // TODO: maybe allow omitting http:
        arg!(--("public-endpoint")|("public-endpoints") =["SPEC"]...)
            .help("An endpoint that will be publicly exposed by instances of this Formation Plan in the form of 'ROUTE=FLIGHT:PORT' (supports comma separated list, or multiple uses)")
            .long_help(LONG_PUBLIC_ENDPOINT)
            .validator(validate_public_endpoint),
        // TODO: maybe allow omitting the Flight's port if it's the same
        arg!(--("flight-endpoint")|("flight-endpoints") =["SPEC"]...)
            .validator(validate_endpoint)
            .help("An endpoint that will only be privately exposed on Instances of this Formation Plan to Flights within the same Formation Instance. In the form of 'PROTO:TARGET=FLIGHT:PORT' (supports comma separated list, or multiple uses)")
            .long_help(LONG_FLIGHT_ENDPOINT),
        arg!(--affinity|affinities =["NAME|ID"]...)
            .help("A Formation that this Formation has an affinity for (supports comma separated list, or multiple uses)")
            .long_help(LONG_AFFINITY)
            .validator(validator)
            .hide(hide), // Hidden on feature = unstable
        arg!(--connection|connections =["NAME|ID"]...)
            .help("A Formations that this Formation is connected to (supports comma separated list, or multiple uses)")
            .long_help(LONG_CONNECTION)
            .validator(validator)
            .hide(hide), // Hidden on feature = unstable
        // TODO: maybe allow omitting the Flight's port if it's the same
        arg!(--("formation-endpoint")|("formation-endpoints") =["SPEC"]...)
            .validator(validate_endpoint)
            .help("An endpoints that will only be exposed privately on Instances of this Formation Plan to other Formations within the same tenant and who have declared mutual connections. In the form of 'PROTO:TARGET=FLIGHT:PORT' (supports comma separated list, or multiple uses)")
            .long_help(LONG_FORMATION_ENDPOINT)
            .hide(hide), // Hidden on feature = unstable
    ]
}
