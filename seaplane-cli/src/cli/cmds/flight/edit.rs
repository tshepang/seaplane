use anyhow::Result;
use clap::Parser;

use seaplane::api::v1::Flight as FlightModel;

use crate::{
    cli::cmds::flight::{SeaplaneFlightCommonArgs, IMAGE_SPEC},
    context::{Ctx, FlightCtx},
    data::flight::{Flight, Flights},
    printer::Printer,
};

// TODO: add --no-maximum or similar
// TODO: add --from
/// Edit a Flight definition
#[derive(Parser)]
#[clap(visible_aliases = &["clone"], after_help = IMAGE_SPEC, override_usage =
"seaplane flight edit <NAME|ID> [OPTIONS]")]
pub struct SeaplaneFlightEditArgs {
    /// The source name or ID of the Flight to copy
    #[clap(value_name = "NAME|ID")]
    source: String,

    /// the given SOURCE must be an exact match
    #[clap(short = 'x', long)]
    exact: bool,

    // So we don't have to define the same args over and over with commands that use the same ones
    #[clap(flatten)]
    shared: SeaplaneFlightCommonArgs,
}

impl SeaplaneFlightEditArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        Printer::init(ctx.color);

        // Load the known Flights from the local JSON "DB"
        let mut flights = Flights::load_from_disk(ctx.flights_file())?.unwrap_or_default();

        // Check for duplicates and suggest `seaplane flight edit`
        // TODO: We should check if these ones we remove are referenced remote or not

        // Ensure only one current Flight matches what the user gave
        let indices = if self.exact {
            flights.indices_of_matches(&self.source)
        } else {
            flights.indices_of_left_matches(&self.source)
        };
        match indices.len() {
            0 => {
                cli_eprint!(@Red, "error: ");
                cli_eprint!("the NAME or ID '");
                cli_eprint!(@Green, "{}", self.source);
                cli_eprintln!("' didn't match anything");
                if self.exact {
                    cli_eprint!("(hint: remove '");
                    cli_eprint!(@Yellow, "--exact");
                    cli_eprintln!("' to allow partial matches)");
                }

                std::process::exit(1);
            }
            1 => (),
            _ => {
                cli_eprint!(@Red, "error: ");
                cli_eprint!("the NAME or ID '");
                cli_eprint!(@Yellow, "{}", self.source);
                cli_eprintln!("' is ambiguous and matches more than one item");
                std::process::exit(1);
            }
        }
        // Temporarily remove that flight so we can do some comparing and copying
        let src_flight = flights.remove_indices(&indices).pop().unwrap();

        // Check if the user provided a value and override source
        // TODO: yuck
        let mut dest_flight = FlightModel::builder();
        if let Some(name) = &self.shared.name {
            dest_flight = dest_flight.name(name);
        } else {
            dest_flight = dest_flight.name(src_flight.model.name());
        }
        if let Some(mut image) = self.shared.image.clone() {
            // TODO: TEMPORARY FIX until bug in ImageReference parsing is fixed
            // (https://github.com/seaplane-io/eng/issues/1847)
            if !image.domain.contains('.') {
                image.path = format!("{}/{}", image.domain, image.path);
                image.domain = "registry.seaplanet.io".into();
            }
            dest_flight = dest_flight.image_reference(image);
        } else {
            dest_flight = dest_flight.image_reference(src_flight.model.image().clone());
        }
        if self.shared.minimum > 0 {
            dest_flight = dest_flight.minimum(self.shared.minimum);
        } else if src_flight.model.minimum() > 0 {
            dest_flight = dest_flight.minimum(src_flight.model.minimum());
        }
        if let Some(max) = self.shared.maximum {
            dest_flight = dest_flight.maximum(max);
        } else if let (Some(max), false) = (src_flight.model.maximum(), self.no_maximum) {
            dest_flight = dest_flight.maximum(max);
        }
        // TODO: we don't define what should happen if both the src and CLI provided differing
        // values here. Does it append, or overwrite? For now we append.
        for arch in self
            .shared
            .architecture
            .iter()
            .chain(src_flight.model.architecture())
        {
            dest_flight = dest_flight.add_architecture(*arch);
        }
        let cli_api_perms = self.shared.api_permission;
        let src_api_perms = src_flight.model.api_permission();
        match (src_api_perms, cli_api_perms) {
            (true, false) => dest_flight = dest_flight.api_permission(false),
            (false, true) => dest_flight = dest_flight.api_permission(true),
            _ => (),
        }

        // Add the new Flight
        // TODO: don't expect
        let flight = Flight {
            id: src_flight.id,
            model: dest_flight.build().expect("Failed to build new Flight"),
        };
        flights.inner.push(flight);

        // Write out an entirely new JSON file with the new Flight included
        flights.save_to_disk(ctx.flights_file())?;

        cli_print!("Successfully editted Flight '");
        cli_print!(@Yellow, "{}", self.source);
        cli_println!("'");

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.flight.init(self.flight_ctx());
        Ok(())
    }

    fn flight_ctx(&self) -> FlightCtx {
        self.shared.flight_ctx()
    }
}
