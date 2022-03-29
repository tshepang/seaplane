use std::collections::HashSet;

use seaplane::api::v1::formations::{
    FormationConfiguration as FormationConfigurationModel, Provider as ProviderModel,
    Region as RegionModel,
};

use crate::{
    cli::{
        cmds::formation::SeaplaneFormationCreateArgMatches, specs::FLIGHT_SPEC, Provider, Region,
    },
    context::Ctx,
    error::{CliErrorKind, Context, Result},
    fs::FromDisk,
    ops::{flight::Flights, formation::Endpoint, generate_formation_name},
    printer::Color,
};

/// Represents the "Source of Truth" i.e. it combines all the CLI options, ENV vars, and config
/// values into a single structure that can be used later to build models for the API or local
/// structs for serializing
///
/// A somewhat counter-intuitive thing about Formations and their models is the there is no
/// "Formation Model" only a "Formation Configuration Model" This is because a "Formation" so to
/// speak is really just a named collection of configurations and info about their traffic
/// weights/activation statuses.
// TODO: we may not want to derive this we implement circular references
#[derive(Debug)]
pub struct FormationCtx {
    pub name_id: String,
    pub launch: bool,
    pub deploy: bool,
    pub remote: bool,
    pub local: bool,
    pub fetch: bool,
    pub grounded: bool,
    // TODO: make multiple possible
    pub cfg_ctx: FormationCfgCtx,
}

impl Default for FormationCtx {
    fn default() -> Self {
        Self {
            name_id: generate_formation_name(),
            launch: false,
            deploy: false,
            cfg_ctx: FormationCfgCtx::default(),
            remote: false,
            local: true,
            fetch: false,
            grounded: false,
        }
    }
}

impl FormationCtx {
    /// `flight` is the name of the argument for the Flight's name/id
    pub fn from_formation_create(
        matches: &SeaplaneFormationCreateArgMatches,
        ctx: &Ctx,
    ) -> Result<Self> {
        let matches = matches.0;
        // TODO: check if "all" was used along with another value within regions/providers and err

        let mut flight_names = Vec::new();

        // TODO: fetch remote flights too
        // Load known local flights
        let flights_file = ctx.flights_file();
        let flights: Flights = FromDisk::load(&flights_file)?;

        // Translate the flight NAME|ID into a NAME, or create the flight when @path or @- is used
        // and save the NAME
        for flight in matches
            .values_of("flight")
            .unwrap_or_default()
            // Filter out @ strings because those flights must be created which after the formation
            // context is created
            .filter(|f| !f.starts_with('@'))
        {
            // Try to lookup either a partial ID match, or exact NAME match
            if let Some(flight) = flights.find_name_or_partial_id(flight) {
                // Look for exact name matches, or partial ID matches and map to their name
                flight_names.push(flight.model.name().to_owned());
            } else {
                // No match
                return Err(CliErrorKind::NoMatchingItem(flight.to_string())
                    .into_err()
                    .context("(hint: create the Flight with '")
                    .with_color_context(|| {
                        (Color::Green, format!("seaplane flight create {flight}"))
                    })
                    .context("')\n")
                    .context("(hint: or try fetching remote references with '")
                    .color_context(Color::Green, "seaplane formation fetch-remote")
                    .context("')\n"));
            }
        }

        let launch = matches.is_present("launch");
        // If we're launching we also need to deploy
        if launch && matches.is_present("no-deploy") {
            return Err(CliErrorKind::ConflictingArguments("--no-deploy", "--launch").into_err());
        }
        let deploy = matches.is_present("deploy") || launch;
        Ok(FormationCtx {
            name_id: matches
                .value_of("name_id")
                .map(ToOwned::to_owned)
                .unwrap_or_else(generate_formation_name),
            deploy: deploy || launch,
            launch: deploy && !matches.is_present("no-launch"),
            cfg_ctx: FormationCfgCtx {
                flights: flight_names.iter().map(|s| s.to_string()).collect(),
                affinities: matches
                    .values_of("affinity")
                    .unwrap_or_default()
                    .map(ToOwned::to_owned)
                    .collect(),
                connections: matches
                    .values_of("connection")
                    .unwrap_or_default()
                    .map(ToOwned::to_owned)
                    .collect(),
                providers_allowed: values_t_or_exit!(@into_model matches, "provider", Provider),
                providers_denied: values_t_or_exit!(@into_model matches, "exclude-provider", Provider),
                regions_allowed: values_t_or_exit!(@into_model matches, "region", Region),
                regions_denied: values_t_or_exit!(@into_model matches, "exclude-region", Region),
                public_endpoints: values_t_or_exit!(matches, "public-endpoint", Endpoint),
                formation_endpoints: values_t_or_exit!(matches, "formation-endpoint", Endpoint),
                flight_endpoints: values_t_or_exit!(matches, "flight-endpoint", Endpoint),
            },
            ..Default::default()
        })
    }

    /// Creates a new seaplane::api::v1::FormationConfiguration from the contained values
    pub fn configuration_model(&self, ctx: &Ctx) -> Result<Option<FormationConfigurationModel>> {
        // If anything other than a name was provided, at least one flight is required too.
        if self.cfg_ctx.flights.is_empty()
            && (!self.cfg_ctx.affinities.is_empty()
                || !self.cfg_ctx.connections.is_empty()
                || !self.cfg_ctx.providers_allowed.is_empty()
                || !self.cfg_ctx.providers_denied.is_empty()
                || !self.cfg_ctx.regions_allowed.is_empty()
                || !self.cfg_ctx.regions_denied.is_empty()
                || !self.cfg_ctx.public_endpoints.is_empty()
                || !self.cfg_ctx.formation_endpoints.is_empty()
                || !self.cfg_ctx.flight_endpoints.is_empty())
        {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("when providing formation configuration options at least one ");
            cli_eprint!(@Green, "--flight=<SPEC>... ");
            cli_eprintln!("is required");
            cli_eprintln!("\n{FLIGHT_SPEC}");
            std::process::exit(1);
        }

        // Create the new Formation model from the CLI inputs
        let mut f_model = FormationConfigurationModel::builder();

        let flights: Flights = FromDisk::load(ctx.flights_file())?;

        for flight_name in &self.cfg_ctx.flights {
            let flight = flights.find_name(flight_name).ok_or_else(|| {
                CliErrorKind::NoMatchingItem(flight_name.to_string())
                    .into_err()
                    .context("(hint: create the Flight with '")
                    .with_color_context(|| {
                        (
                            Color::Green,
                            format!("seaplane flight create {flight_name}"),
                        )
                    })
                    .context("')\n")
                    .context("(hint: or try fetching remote references with '")
                    .color_context(Color::Green, "seaplane formation fetch-remote")
                    .context("')\n")
            })?;
            f_model = f_model.add_flight(flight.model.clone());
        }

        // TODO: clean this up...yuck
        for item in &self.cfg_ctx.affinities {
            f_model = f_model.add_affinity(item);
        }
        for item in &self.cfg_ctx.connections {
            f_model = f_model.add_connection(item);
        }
        for &item in &self.cfg_ctx.providers_allowed {
            f_model = f_model.add_allowed_provider(item);
        }
        for &item in &self.cfg_ctx.providers_denied {
            f_model = f_model.add_denied_provider(item);
        }
        for &item in &self.cfg_ctx.regions_allowed {
            f_model = f_model.add_allowed_region(item);
        }
        for &item in &self.cfg_ctx.regions_denied {
            f_model = f_model.add_denied_region(item);
        }
        for item in &self.cfg_ctx.public_endpoints {
            f_model = f_model.add_public_endpoint(item.key(), item.value());
        }
        for item in &self.cfg_ctx.formation_endpoints {
            f_model = f_model.add_formation_endpoint(item.key(), item.value());
        }
        for item in &self.cfg_ctx.flight_endpoints {
            f_model = f_model.add_flight_endpoint(item.key(), item.value());
        }

        // TODO: probably match and check errors
        // Create a new Formation struct we can add to our local JSON "DB"
        Ok(f_model.build().ok())
    }
}

#[derive(Default, Debug, Clone)]
pub struct FormationCfgCtx {
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub flights: Vec<String>,
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub affinities: Vec<String>,
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub connections: Vec<String>,
    /// Use actual API model since that is ultimately what we want
    pub providers_allowed: HashSet<ProviderModel>,
    /// Use actual API model since that is ultimately what we want
    pub providers_denied: HashSet<ProviderModel>,
    /// Use actual API model since that is ultimately what we want
    pub regions_allowed: HashSet<RegionModel>,
    /// Use actual API model since that is ultimately what we want
    pub regions_denied: HashSet<RegionModel>,
    pub public_endpoints: Vec<Endpoint>,
    pub formation_endpoints: Vec<Endpoint>,
    pub flight_endpoints: Vec<Endpoint>,
}
