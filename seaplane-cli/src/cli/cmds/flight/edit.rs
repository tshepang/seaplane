use clap::Parser;



use crate::{
    cli::{
        cmds::flight::{SeaplaneFlightCommonArgs, IMAGE_SPEC},
        errors::wrap_cli_context,
    },
    context::{Ctx, FlightCtx},
    error::Result,
    ops::flight::{Flights},
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
        let flights_file = ctx.flights_file();
        let mut flights = Flights::load_from_disk(&flights_file)?;

        // Now we just edit the newly copied Flight to match the given CLI params...
        let flight_ctx = self.shared.flight_ctx();
        if let Err(e) = flights.update_flight(&self.source, self.exact, &flight_ctx) {
            return wrap_cli_context(e, self.exact, false);
        }

        // Write out an entirely new JSON file with the new Flight included
        flights.save_to_disk()?;

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
