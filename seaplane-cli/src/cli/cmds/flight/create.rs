use anyhow::Result;
use clap::Parser;

use crate::{
    cli::cmds::flight::{SeaplaneFlightCommonArgs, IMAGE_SPEC},
    context::FlightCtx,
    data::flight::{Flight, Flights},
    printer::Printer,
    Ctx,
};

// TODO: add --from
/// Create a new Flight definition
#[derive(Parser)]
#[clap(visible_aliases = &["add"], after_help = IMAGE_SPEC, override_usage =
"seaplane flight create --image=<IMAGE_SPEC> [OPTIONS]")]
pub struct SeaplaneFlightCreateArgs {
    // So we don't have to define the same args over and over with commands that use the same ones
    #[clap(flatten)]
    shared: SeaplaneFlightCommonArgs,

    /// Override any existing Flights with the same <NAME>
    #[clap(short, long)]
    force: bool,
}

impl SeaplaneFlightCreateArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        Printer::init(ctx.color);

        // In the shared args --image is optional, because not all commands need it to be
        // mandatory.
        //
        // So we have to check ourselves to make sure it's included, and can unwrap() down below.
        if self.shared.image.is_none() {
            // We emulate clap errors so as to provide a cohesive experience
            cli_print!(@Red, "error: ");
            cli_println!("The following required arguments were not provided:");
            cli_println!(@Green, "    --image=<IMAGE_SPEC>");
            cli_println!("");
            cli_println!("USAGE:");
            cli_println!("seaplane flight create --image=<IMAGE_SPEC> [OPTIONS]");
            cli_println!("");
            cli_print!("For more information try ");
            cli_println!(@Green, "--help");
            std::process::exit(2);
        }

        let new_flight = ctx.flight_ctx().model();

        // Load the known Flights from the local JSON "DB"
        let mut flights = Flights::load_from_disk(ctx.flights_file())?.unwrap_or_default();

        // Check for duplicates and suggest `seaplane flight edit`
        if flights.is_ambiguous_or_die(new_flight.name(), !self.force) {
            // TODO: We should check if these ones we remove are referenced remote or not

            // We have duplicates, but the user passed --force. So first we remove the existing
            // Flights and "re-add" them

            flights.remove_indices(&flights.indices_of_matches(new_flight.name()));
        }

        let new_flight_name = new_flight.name().to_owned();
        // Add the new Flight
        let new_flight = Flight::new(new_flight);
        let id = hex::encode(new_flight.id);
        flights.inner.push(new_flight);

        // Write out an entirely new JSON file with the new Flight included
        flights.save_to_disk(ctx.flights_file())?;

        cli_print!("Successfully created Flight '");
        cli_print!(@Green, "{}", new_flight_name);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
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