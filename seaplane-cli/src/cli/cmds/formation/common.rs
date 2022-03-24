//! The common args effectively represent a "Formation Configuration" which many commands can
//! include.
//!
//! The only additional information is the formation name, which is not part of the configuration,
//! but many commands need as well.

use clap::Arg;

use seaplane::api::v1::{Provider as ProviderModel, Region as RegionModel};
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::cli::validator::{
    validate_endpoint, validate_name, validate_name_id, validate_name_id_path,
};

static LONG_NAME: &str =
    "A human readable name for the Formation (must be unique within the tenant)

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future.";
static LONG_AFFINITY: &str = "A Formation that this Formation has an affinity for.

This is a hint to the scheduler to place containers run in each of these
formations \"close\" to eachother (for some version of close including but
not limited to latency).";
static LONG_CONNECTION: &str = "A Formations that this Formation is connected to.

Two formations can communicate over their formation endpoints (the endpoints configured via
--formation-endpoints) if and only if both formations opt in to that connection (list
each other in their connections map)";

static LONG_PUBLIC_ENDPOINT: &str = r#"A publicly exposed endpoints of this Formations

Public Endpoints take the form 'http:{ROUTE}={FLIGHT}:{PORT}'. Where

ROUTE  := An HTTP URL route
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this endpoint's route should be sent.

For example, consider:

$ seaplane formation edit Foo --public-endpoint=http:/foo/bar=baz:1234

Would mean, route all traffic from the public internet arriving at the path 
'/foo/bar' on the 'Foo' Formation's domain to this Formation's Flight named 
'baz' on port '1234'

In the future, support for other protocols may be added in place of 'http'
"#;

static LONG_FORMATION_ENDPOINT: &str = r#"A privately exposed endpoint of this Formations (only expose to other Formations)

Formation Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where

PROTO  := http | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this Formation's endpoint should be sent.

For example, consider:

$ seaplane formation edit Foo --formation-endpoint=tcp:22=baz:2222

Would mean, route all traffic from the private network arriving on TCP/22 on the 'Foo' Formation's
domain to the this Formation's Flight named 'baz' on port '2222'. The PROTO of the incoming traffic
will be used for the PROTO of the outgoing traffic to FLIGHT
"#;

static LONG_FLIGHT_ENDPOINT: &str = r#"A privately exposed endpoint of this Formations (only expose to other
Flights within this Formation)

Formation Endpoints take the form '{PROTO}:{TARGET}={FLIGHT}:{PORT}'. Where

PROTO  := http | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this Formation's endpoint should be sent.

For example, consider:

$ seaplane formation edit Foo --flight-endpoint=udp:1234=baz:4321

Would mean, route all traffic from the Formation's private network arriving on UDP/1234 on the
'Foo' Formation's domain to the this Formation's Flight named 'baz' on port '4321'. The PROTO of
the incoming traffic will be used for the PROTO of the outgoing traffic to FLIGHT
"#;

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
    // TODO: add --from with support for @file and @- (stdin)
    vec![
        arg!(name_id --name -('n') =["STRING"])
            .help("A human readable name for the Formation (must be unique within the tenant) if omitted a pseudo random name will be assigned")
            .long_help(LONG_NAME)
            .validator(validate_name),
        arg!(--launch|active)
            .help("This Formation configuration should be deployed and set it as active right away (requires a formation configuration)")
            .overrides_with("no-launch"),
        arg!(--("no-launch")|("no-active"))
            .help("The opposite of --launch, and says that this Formation should not be active")
            .overrides_with("launch"),
        arg!(--deploy)
                .help("Send this formation to Seaplane immediately (requires a Formation configuration) but DO NOT set to active. To also set the configuration to active add or use --launch instead")
                .overrides_with("no-deploy"),
        arg!(--("no-deploy"))
            .overrides_with_all(&["deploy", "launch"])
            .help("Do *not* send this formation to Seaplane immediately"),
        arg!(--flight|flights =["SPEC"]...)
            .help("A Flight to add to this formation in the form of ID|NAME|@path|@- (See FLIGHT SPEC below)")
            .validator(validate_name_id_path),
        arg!(--affinity|affinities =["NAME|ID"]...)
            .help("A Formation that this Formation has an affinity for")
            .long_help(LONG_AFFINITY)
            .validator(validate_name_id),
        arg!(--connection|connections =["NAME|ID"]...)
            .help("A Formations that this Formation is connected to")
            .long_help(LONG_CONNECTION)
            .validator(validate_name_id),
        arg!(--provider|providers =["PROVIDER"=>"all"]... ignore_case)
            .help("A provider that this Formation's Flights are permitted to run on")
            .possible_values(Provider::VARIANTS),
        arg!(--("exclude-provider")|("exclude-providers") =["PROVIDER"]... ignore_case)
            .help("A provider that this Formation's Flights are *NOT* permitted to run on. This will override any matching value given by via --provider")
            .possible_values(Provider::VARIANTS),
        arg!(--region|regions =["REGION"=>"all"]... ignore_case)
            .help("A region in which this Formation's Flights are allowed to run in (See REGION SPEC below)")
            .possible_values(Region::VARIANTS),
        arg!(--("exclude-region")|("exclude-regions") =["REGION"]... ignore_case)
            .help("A region in which this Formation's Flights are *NOT* allowed to run in (See REGION SPEC below)")
            .possible_values(Region::VARIANTS),
        // TODO: maybe allow omitting http:
        arg!(--("public-endpoint")|("public-endpoints") =["SPEC"]...)
            .help("A publicly exposed endpoints of this Formation in the form of 'http:ROUTE=FLIGHT:PORT'")
            .long_help(LONG_PUBLIC_ENDPOINT)
            .validator(validate_endpoint),
        // TODO: maybe allow omitting the Flight's port if it's the same
        arg!(--("formation-endpoint")|("formation-endpoints") =["SPEC"]...)
            .validator(validate_endpoint)
            .help("An endpoints exposed only to other Formations privately. In the form of 'PROTO:TARGET=FLIGHT:PORT'")
            .long_help(LONG_FORMATION_ENDPOINT),
        // TODO: maybe allow omitting the Flight's port if it's the same
        arg!(--("flight-endpoint")|("flight-endpoints") =["SPEC"]...)
            .validator(validate_endpoint)
            .help("An endpoint exposed only to Flights within this Formation. In the form of 'PROTO:TARGET=FLIGHT:PORT'")
            .long_help(LONG_FLIGHT_ENDPOINT),
    ]
}
