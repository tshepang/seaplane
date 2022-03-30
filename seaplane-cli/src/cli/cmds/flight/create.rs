use clap::{ArgMatches, Command};

use crate::{
    cli::{
        cmds::flight::{
            common::{self, SeaplaneFlightCommonArgMatches},
            IMAGE_SPEC,
        },
        CliCommand,
    },
    context::FlightCtx,
    error::{CliErrorKind, Context, Result},
    fs::{FromDisk, ToDisk},
    ops::flight::{Flight, Flights},
    printer::Color,
    Ctx,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightCreate;

impl SeaplaneFlightCreate {
    pub fn command() -> Command<'static> {
        // TODO: add --from
        Command::new("create")
            .visible_alias("add")
            .after_help(IMAGE_SPEC)
            .override_usage("seaplane flight create --image=<SPEC> [OPTIONS]")
            .about("Create a new Flight definition")
            .arg(arg!(--force - ('f')).help("Override any existing Flights with the same NAME"))
            .args(common::args(true))
    }
}

impl CliCommand for SeaplaneFlightCreate {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let new_flight = ctx.flight_ctx.get_or_init().model();

        // Load the known Flights from the local JSON "DB"
        let flights_file = ctx.flights_file();
        let mut flights: Flights = FromDisk::load(&flights_file)?;

        // Check for duplicates and suggest `seaplane flight edit`
        let name = new_flight.name();
        let indices = flights.indices_of_matches(name);
        if !indices.is_empty() {
            // TODO: We should check if these ones we remove are referenced remote or not

            if !ctx.args.force {
                return Err(CliErrorKind::DuplicateName(name.into())
                    .into_err()
                    .context("(hint: try '")
                    .color_context(Color::Green, format!("seaplane flight edit {name}"))
                    .context("' instead)\n"));
            }

            // We have duplicates, but the user passed --force. So first we remove the existing
            // Flights and "re-add" them

            // TODO: if more than one flight has the exact same name, we remove them all; that's
            // *probably* what we want? But more thought should go into this...
            flights.remove_indices(&indices);
        }

        let new_flight_name = new_flight.name().to_owned();
        // Add the new Flight
        let new_flight = Flight::new(new_flight);
        let id = new_flight.id;
        flights.inner.push(new_flight);

        // Write out an entirely new JSON file with the new Flight included
        flights.persist()?;

        cli_print!("Successfully created Flight '");
        cli_print!(@Green, "{new_flight_name}");
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id.to_string()[..8]);
        cli_println!("'");

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.flight_ctx.init(FlightCtx::from_flight_common(
            &SeaplaneFlightCommonArgMatches(matches),
            "",
        )?);
        ctx.args.force = matches.is_present("force");
        Ok(())
    }
}
