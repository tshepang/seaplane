use clap::Parser;

use crate::{
    cli::{cmds::flight::SeaplaneFlightCommonArgs, errors::wrap_cli_context, specs::IMAGE_SPEC},
    context::Ctx,
    error::Result,
    fs::{FromDisk, ToDisk},
    ops::flight::Flights,
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

        // Load the known Flights from the local JSON "DB"
        let flights_file = ctx.flights_file();
        let mut flights: Flights = FromDisk::load(&flights_file)?;

        let mut dest_flight = match flights.clone_flight(&self.source, self.exact) {
            Ok(f) => f,
            Err(e) => return wrap_cli_context(e, self.exact, false),
        };

        // Now we just edit the newly copied Flight to match the given CLI params...
        dest_flight.update_from(&ctx.flight_ctx(), false)?;

        let id = dest_flight.id.to_string();
        let name = dest_flight.model.name().to_owned();

        // Add the new Flight
        flights.inner.push(dest_flight);

        // Write out an entirely new JSON file with the new Flight included
        flights.persist()?;

        cli_print!("Successfully copied Flight '");
        cli_print!(@Yellow, "{}", self.source);
        cli_print!("' to new Flight '");
        cli_print!(@Green, "{}", name);
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
        cli_println!("'");

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.flight.init(self.shared.flight_ctx()?);
        Ok(())
    }
}
