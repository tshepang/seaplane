use std::collections::HashSet;

use seaplane::api::v1::formations::{
    FormationConfiguration as FormationConfigurationModel, Provider as ProviderModel,
    Region as RegionModel,
};

use crate::{
    cli::specs::FLIGHT_SPEC,
    context::Ctx,
    error::{CliErrorKind, Context, Result},
    fs::FromDisk,
    ops::{flight::Flights, formation::Endpoint, generate_name},
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
pub struct FormationCtx {
    pub name: String,
    pub take_off: bool,
    pub deploy: bool,
    pub remote: bool,
    pub local: bool,
    // TODO: make multiple possible
    pub cfg_ctx: FormationCfgCtx,
}

impl Default for FormationCtx {
    fn default() -> Self {
        Self {
            name: generate_name(),
            take_off: false,
            deploy: false,
            cfg_ctx: FormationCfgCtx::default(),
            remote: false,
            local: true,
        }
    }
}

impl FormationCtx {
    /// Creates a new seaplane::api::v1::FormationConfiguration from the contained values
    pub fn configuration_model(&self, ctx: &Ctx) -> Result<Option<FormationConfigurationModel>> {
        // If anything other than a name was provided, at least one flight is required too.
        if self.cfg_ctx.flight.is_empty()
            && (!self.cfg_ctx.affinity.is_empty()
                || !self.cfg_ctx.connection.is_empty()
                || !self.cfg_ctx.providers_allowed.is_empty()
                || !self.cfg_ctx.providers_denied.is_empty()
                || !self.cfg_ctx.regions_allowed.is_empty()
                || !self.cfg_ctx.regions_denied.is_empty()
                || !self.cfg_ctx.public_endpoint.is_empty()
                || !self.cfg_ctx.formation_endpoint.is_empty()
                || !self.cfg_ctx.flight_endpoint.is_empty())
        {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("when providing formation configuration options at least one ");
            cli_eprint!(@Green, "--flight=<SPEC>... ");
            cli_eprintln!("is required");
            cli_eprintln!("\n{}", FLIGHT_SPEC);
            std::process::exit(1);
        }

        // Create the new Formation model from the CLI inputs
        let mut f_model = FormationConfigurationModel::builder();

        let flights: Flights = FromDisk::load(ctx.flights_file())?;

        for flight_name in &self.cfg_ctx.flight {
            let flight = flights.find_name(flight_name).ok_or_else(|| {
                CliErrorKind::NoMatchingItem(flight_name.to_string())
                    .into_err()
                    .context("(hint: create the Flight with '")
                    .with_color_context(|| {
                        (
                            Color::Green,
                            format!("seaplane flight create {}", flight_name),
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
        for item in &self.cfg_ctx.affinity {
            f_model = f_model.add_affinity(item);
        }
        for item in &self.cfg_ctx.connection {
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
        for item in &self.cfg_ctx.public_endpoint {
            f_model = f_model.add_public_endpoint(item.key(), item.value());
        }
        for item in &self.cfg_ctx.formation_endpoint {
            f_model = f_model.add_formation_endpoint(item.key(), item.value());
        }
        for item in &self.cfg_ctx.flight_endpoint {
            f_model = f_model.add_flight_endpoint(item.key(), item.value());
        }

        // TODO: probably match and check errors
        // Create a new Formation struct we can add to our local JSON "DB"
        Ok(f_model.build().ok())
    }
}

#[derive(Default, Debug, Clone)]
pub struct FormationCfgCtx {
    pub take_off: bool,
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub flight: Vec<String>,
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub affinity: Vec<String>,
    /// `String` is a flight name because that's the only thing shared by both local and remote
    pub connection: Vec<String>,
    /// Use actual API model since that is ultimately what we want
    pub providers_allowed: HashSet<ProviderModel>,
    /// Use actual API model since that is ultimately what we want
    pub providers_denied: HashSet<ProviderModel>,
    /// Use actual API model since that is ultimately what we want
    pub regions_allowed: HashSet<RegionModel>,
    /// Use actual API model since that is ultimately what we want
    pub regions_denied: HashSet<RegionModel>,
    pub public_endpoint: Vec<Endpoint>,
    pub formation_endpoint: Vec<Endpoint>,
    pub flight_endpoint: Vec<Endpoint>,
}
