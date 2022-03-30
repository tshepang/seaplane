use clap::{ArgMatches, Command};

use crate::{
    cli::{
        cmds::flight::common,
        errors::wrap_cli_context,
        specs::IMAGE_SPEC,
        validator::{validate_flight_name, validate_name_id},
        CliCommand,
    },
    context::{Ctx, FlightCtx},
    error::Result,
};

use super::SeaplaneFlightCommonArgMatches;

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightCopy;

impl SeaplaneFlightCopy {
    pub fn command() -> Command<'static> {
        let validator = |s: &str| validate_name_id(validate_flight_name, s);

        // TODO: add --from
        Command::new("copy")
            .visible_alias("clone")
            .about("Copy a Flight definition")
            .after_help(IMAGE_SPEC)
            .override_usage(
                "seaplane flight copy <NAME|ID> --name=<DEST_NAME> [OPTIONS]
    seaplane flight copy <NAME|ID> [OPTIONS]",
            )
            .arg(
                arg!(name_id =["NAME|ID"] required)
                    .validator(validator)
                    .help("The source name or ID of the Flight to copy"),
            )
            .arg(arg!(--exact - ('x')).help("The given SOURCE must be an exact match"))
            .args(common::args(false))
    }
}

impl CliCommand for SeaplaneFlightCopy {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.stateless {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("'");
            cli_eprint!(@Yellow, "--stateless");
            cli_eprint!("' cannot be used with the '");
            cli_eprint!(@Yellow, "seaplane flight copy");
            cli_eprintln!("' command");
            cli_eprintln!("(hint: 'seaplane flight ...' only modifies local state)");
            std::process::exit(1);
        }
        // name_id cannot be None in `flight copy`
        let mut dest_flight = match ctx
            .db
            .flights
            .clone_flight(ctx.args.name_id.as_ref().unwrap(), ctx.args.exact)
        {
            Ok(f) => f,
            Err(e) => return wrap_cli_context(e, ctx.args.exact, false),
        };

        // Now we just edit the newly copied Flight to match the given CLI params...
        dest_flight.update_from(&ctx.flight_ctx.get_or_init(), false)?;

        let id = dest_flight.id.to_string();
        let name = dest_flight.model.name().to_owned();

        // Add the new Flight
        ctx.db.flights.add_flight(dest_flight);

        ctx.persist_flights()?;

        cli_print!("Successfully copied Flight '");
        cli_print!(@Yellow, "{}", ctx.args.name_id.as_ref().unwrap());
        cli_print!("' to new Flight '");
        cli_print!(@Green, "{name}");
        cli_print!("' with ID '");
        cli_print!(@Green, "{}", &id[..8]);
        cli_println!("'");

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        // clap will not let "source" be None
        ctx.args.name_id = matches.value_of("name_id").map(ToOwned::to_owned);
        ctx.flight_ctx.init(FlightCtx::from_flight_common(
            &SeaplaneFlightCommonArgMatches(matches),
            "",
        )?);
        Ok(())
    }
}
