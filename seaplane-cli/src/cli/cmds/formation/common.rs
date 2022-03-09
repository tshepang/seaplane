//! The common args effectively represent a "Formation Configuration" which many commands can
//! include.
//!
//! The only additional information is the formation name, which is not part of the configuration,
//! but many commands need as well.

use std::{
    fs,
    io::{self, Read},
};

use clap::Parser;
use const_format::concatcp;
use seaplane::api::v1::{Provider as ProviderModel, Region as RegionModel};
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::{
    cli::{
        specs::{FLIGHT_SPEC, REGION_SPEC},
        validator::{validate_name, validate_name_id, validate_name_id_path},
    },
    context::{Ctx, FormationCfgCtx, FormationCtx},
    error::{CliError, CliErrorKind, Context, Result},
    fs::FromDisk,
    ops::{
        flight::{Flight, Flights},
        formation::Endpoint,
        generate_name,
    },
    printer::Color,
};

/// We provide a shim between the Seaplane Provider so we can do some additional UX work like 'all'
#[derive(Debug, Copy, Clone, PartialEq, EnumString, EnumVariantNames)]
#[strum(ascii_case_insensitive)]
pub enum Provider {
    Aws,
    Azure,
    DigitalOcean,
    Equinix,
    Gcp,
    All,
}

impl Provider {
    fn try_into(self) -> Option<ProviderModel> {
        if self == Provider::All {
            None
        } else {
            Some(Provider::into(self))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ProviderModel> for Provider {
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

/// We provide a shim between the Seaplane Region so we can do some additional UX work
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, EnumString, EnumVariantNames)]
#[strum(ascii_case_insensitive)]
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
    fn try_into(self) -> Option<RegionModel> {
        if self == Region::All {
            None
        } else {
            Some(Region::into(self))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<RegionModel> for Region {
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

#[derive(Parser)]
#[clap(after_help = concatcp!(FLIGHT_SPEC, "\n\n", REGION_SPEC))]
pub struct SeaplaneFormationCommonArgs {
    // TODO: add --from with support for @file and @- (stdin)
    /// A human readable name for the Formation (must be unique within the tenant) if omitted a
    /// pseudo random name will be assigned
    #[clap(
        short,
        long,
        validator = validate_name,
        long_help = "A human readable name for the Formation (must be unique within the tenant)

Rules for a valid name are as follows:

  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
  - hyphens ('-') may not be repeated (i.e. '--')
  - no more than three (3) total hyphens
  - the total length must be <= 27

Some of these restrictions may be lifted in the future."
    )]
    pub name: Option<String>,

    /// This Formation configuration should be deployed and set it as active right away (requires a
    /// formation configuration)
    #[clap(long, visible_alias = "active", overrides_with = "no-launch")]
    pub launch: bool,

    /// The opposite of --launch, and says that this Formation should not be active
    #[clap(long, visible_alias = "no-active", overrides_with = "launch")]
    pub no_launch: bool,

    /// A Flight to add to this formation in the form of ID|NAME|@path|@- (See FLIGHT SPEC below)
    #[clap(long, value_delimiter = ',', validator = validate_name_id_path, value_name = "SPEC")]
    pub flight: Vec<String>,

    /// A Formation that this Formation has an affinity for
    #[clap(
        long,
        value_delimiter = ',',
        value_name = "NAME|ID",
        visible_alias = "affinities",
        validator = validate_name_id,
        long_help = "A Formation that this Formation has an affinity for.

This is a hint to the scheduler to place containers run in each of these
formations \"close\" to eachother (for some version of close including but
not limited to latency)."
    )]
    pub affinity: Vec<String>,

    /// A Formations that this Formation is connected to.
    ///
    /// Two formations can
    /// communicate over their formation endpoints (the endpoints configured in the
    /// formation_endpoints section) if and only if both formations opt in to that connection (list
    /// each other in their connections map
    #[clap(
        long,
        value_delimiter = ',',
        value_name = "NAME|ID",
        visible_alias = "connections",
        validator = validate_name_id,
        long_help = "A Formations that this Formation is connected to.

Two formations can communicate over their formation endpoints (the endpoints configured via
--formation-endpoints) if and only if both formations opt in to that connection (list
each other in their connections map)")]
    pub connection: Vec<String>,

    /// A provider that this Formation's Flights are permitted to run on
    #[clap(long, default_value = "All", possible_values = Provider::VARIANTS)]
    pub provider: Vec<Provider>,

    /// A provider that this Formation's Flights are *NOT* permitted to run on. This will override any
    /// matching value given by via --provider
    #[clap(long, default_value = "All", value_name = "PROVIDER", possible_values = Provider::VARIANTS)]
    pub exclude_provider: Vec<Provider>,

    /// A region in which this Formation's Flights are allowed to run in (See REGION SPEC below)
    #[clap(long, default_value = "All", possible_values = Region::VARIANTS)]
    pub region: Vec<Region>,

    /// A region in which this Formation's Flights are *NOT* allowed to run in (See REGION SPEC
    /// below)
    #[clap(long, value_name = "REGION", possible_values = Region::VARIANTS)]
    pub exclude_region: Vec<Region>,

    // TODO: maybe allow omitting http:
    /// A publicly exposed endpoints of this Formations in the form of 'http:ROUTE->FLIGHT:PORT'
    #[clap(
        long,
        value_delimiter = ',',
        long_help = r#"A publicly exposed endpoints of this Formations

Public Endpoints take the form 'http:{ROUTE}->{FLIGHT}:{PORT}'. Where

ROUTE  := An HTTP URL route
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this endpoint's route should be sent.

For example, consider:

$ seaplane formation edit Foo --public-endpoint=http:/foo/bar->baz:1234

Would mean, route all traffic from the public internet arriving at the path 
'/foo/bar' on the 'Foo' Formation's domain to this Formation's Flight named 
'baz' on port '1234'

In the future, support for other protocols may be added in place of 'http'
"#
    )]
    pub public_endpoint: Vec<Endpoint>,

    // TODO: maybe allow omitting the Flight's port if it's the same
    /// An endpoints exposed only to other Formations privately. In the form of
    /// 'PROTO:TARGET->FLIGHT:PORT'
    #[clap(
        long,
        value_delimiter = ',',
        long_help = r#"A privately exposed endpoint of this Formations (only expose to other Formations)

Formation Endpoints take the form '{PROTO}:{TARGET}->{FLIGHT}:{PORT}'. Where

PROTO  := http | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this Formation's endpoint should be sent.

For example, consider:

$ seaplane formation edit Foo --formation-endpoint=tcp:22->baz:2222

Would mean, route all traffic from the private network arriving on TCP/22 on the 'Foo' Formation's
domain to the this Formation's Flight named 'baz' on port '2222'. The PROTO of the incoming traffic
will be used for the PROTO of the outgoing traffic to FLIGHT
"#
    )]
    pub formation_endpoint: Vec<Endpoint>,

    // TODO: maybe allow omitting the Flight's port if it's the same
    /// An endpoint exposed only to Flights within this Formation. In the form of
    /// 'PROTO:TARGET->FLIGHT:PORT'
    #[clap(
        long,
        value_delimiter = ',',
        long_help = r#"A privately exposed endpoint of this Formations (only expose to other
Flights within this Formation)

Formation Endpoints take the form '{PROTO}:{TARGET}->{FLIGHT}:{PORT}'. Where

PROTO  := http | tcp | udp
TARGET := ROUTE | PORT
ROUTE  := with PROTO http, and HTTP URL route
PORT   := with PROTO tcp | PROTO udp a Network Port (0-65535)
FLIGHT := NAME or ID
PORT   := Network Port (0-65535)

This describes where traffic arriving at this Formation's endpoint should be sent.

For example, consider:

$ seaplane formation edit Foo --flight-endpoint=udp:1234->baz:4321

Would mean, route all traffic from the Formation's private network arriving on UDP/1234 on the
'Foo' Formation's domain to the this Formation's Flight named 'baz' on port '4321'. The PROTO of
the incoming traffic will be used for the PROTO of the outgoing traffic to FLIGHT
"#
    )]
    pub flight_endpoint: Vec<Endpoint>,
}

impl SeaplaneFormationCommonArgs {
    pub fn formation_ctx(&self, ctx: &Ctx) -> Result<FormationCtx> {
        // TODO: check if "all" was used along with another value within regions/providers and err

        let mut flight_names = Vec::new();

        // Find the name of all flights referenced by self.flight
        let mut already_used_stdin = false;

        // TODO: fetch remote flights too
        // Load known local flights
        let flights_file = ctx.flights_file();
        let mut flights: Flights = FromDisk::load(&flights_file)?;

        // Translate the flight NAME|ID into a NAME, or create the flight when @path or @- is used
        // and save the NAME
        for flight in &self.flight {
            // First try to create for a @- (STDIN)
            if flight == "@-" {
                if already_used_stdin {
                    return Err(CliErrorKind::MultipleAtStdin.into_err());
                }
                already_used_stdin = true;
                let mut buf = String::new();
                let stdin = io::stdin();
                let mut stdin_lock = stdin.lock();
                stdin_lock.read_to_string(&mut buf)?;

                // TODO: we need to check for and handle duplicates
                let new_flight = Flight::from_json(&buf)?;
                flight_names.push(new_flight.model.name().to_owned());
                flights.inner.push(new_flight);
            // next try to create if using @path
            } else if let Some(path) = flight.strip_prefix('@') {
                let new_flight = Flight::from_json(
                    &fs::read_to_string(path)
                        .map_err(CliError::from)
                        .context("\n\tpath: ")
                        .with_color_context(|| (Color::Yellow, path))?,
                )?;
                flight_names.push(new_flight.model.name().to_owned());
                flights.inner.push(new_flight);
            // Finally, try to lookup either a partial ID match, or exact NAME match
            } else if let Some(flight) = flights.find_name_or_partial_id(flight) {
                // Look for exact name matches, or partial ID matches and map to their name
                flight_names.push(flight.model.name().to_owned());
            } else {
                // No match
                return Err(CliErrorKind::NoMatchingItem(flight.to_string())
                    .into_err()
                    .context("(hint: create the Flight with '")
                    .with_color_context(|| {
                        (Color::Green, format!("seaplane flight create {}", flight))
                    })
                    .context("')\n")
                    .context("(hint: or try fetching remote references with '")
                    .color_context(Color::Green, "seaplane formation fetch-remote")
                    .context("')\n"));
            }
        }

        Ok(FormationCtx {
            name: self.name.clone().unwrap_or_else(generate_name),
            launch: self.launch,
            deploy: false,
            cfg_ctx: FormationCfgCtx {
                launch: self.launch,
                flight: flight_names.iter().map(|s| s.to_string()).collect(),
                affinity: self.affinity.clone(),
                connection: self.connection.clone(),
                providers_allowed: self
                    .provider
                    .iter()
                    .copied()
                    .filter_map(Provider::try_into)
                    .collect(),
                providers_denied: self
                    .exclude_provider
                    .iter()
                    .copied()
                    .filter_map(Provider::try_into)
                    .collect(),
                regions_allowed: self
                    .region
                    .iter()
                    .copied()
                    .filter_map(Region::try_into)
                    .collect(),
                regions_denied: self
                    .exclude_region
                    .iter()
                    .copied()
                    .filter_map(Region::try_into)
                    .collect(),
                public_endpoint: self.public_endpoint.clone(),
                formation_endpoint: self.formation_endpoint.clone(),
                flight_endpoint: self.flight_endpoint.clone(),
            },
            ..Default::default()
        })
    }
}
