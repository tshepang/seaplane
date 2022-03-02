use clap::Parser;

use seaplane::api::v1::Flight as FlightModel;

use crate::{
    cli::{
        cmds::flight::{SeaplaneFlightCommonArgs, IMAGE_SPEC},
        errors::wrap_cli_context,
    },
    context::{Ctx, FlightCtx},
    error::{CliErrorKind, Context, Result},
    ops::flight::{generate_name, Flight, Flights},
    printer::{Color, Printer},
};

// TODO: add --from
/// Copy a Flight definition
#[derive(Parser)]
#[clap(visible_aliases = &["clone"], after_help = IMAGE_SPEC, override_usage =
"seaplane flight copy <NAME|ID> --name=<DEST_NAME> [OPTIONS]
    seaplane flight copy <NAME|ID> [OPTIONS]")]
pub struct SeaplaneFlightCopyArgs {
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

impl SeaplaneFlightCopyArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        Printer::init(ctx.color);

        // Load the known Flights from the local JSON "DB"
        let flights_file = ctx.flights_file();
        let mut flights = Flights::load_from_disk(&flights_file)?;

        let mut dest_flight = match flights.clone_flight(&self.source, self.exact) {
            Ok(f) => f,
            Err(e) => return wrap_cli_context(e, self.exact, false),
        };

        // Now we just edit the newly copied Flight to match the given CLI params...
        let flight_ctx = self.shared.flight_ctx();
        dest_flight.update_from(&flight_ctx)?;

        let id = &hex::encode(dest_flight.id)[..8].to_owned();
        let name = dest_flight.model.name().to_owned();

        // Add the new Flight
        flights.inner.push(dest_flight);

        // Write out an entirely new JSON file with the new Flight included
        flights.save_to_disk()?;

        cli_print!("Successfully copied Flight '");
        cli_print!(@Yellow, "{}", self.source);
        cli_print!("' to new Flight '");
        cli_print!(@Green, "{}", name);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", id);
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
