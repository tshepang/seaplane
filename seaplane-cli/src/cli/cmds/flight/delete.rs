use clap::{ArgMatches, Command};

use crate::{
    cli::{
        errors,
        validator::{validate_flight_name, validate_name_id},
        CliCommand,
    },
    context::Ctx,
    error::Result,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightDelete;

impl SeaplaneFlightDelete {
    pub fn command() -> Command<'static> {
        let validator = |s: &str| validate_name_id(validate_flight_name, s);
        // TODO: add a --local[-only] flag or similar that combines with --force to only remove local
        Command::new("delete")
            .visible_aliases(&["del", "remove", "rm"])
            .override_usage("seaplane flight delete <NAME|ID> [OPTIONS]")
            .about("Delete a Flight definition")
            .arg(arg!(flight required =["NAME|ID"])
                .validator(validator)
                .help("The name or ID of the Flight to remove, must be unambiguous"))
            .arg(arg!(--force)
                .help("Delete this Flight even if referenced by a Formation (removes any references in Formations), or deletes ALL Flights referenced by <FLIGHT> even if ambiguous"))
            .arg(arg!(--exact -('x'))
                .conflicts_with("all")
                .help("The given FLIGHT must be an exact match")
                )
            .arg(arg!(--all -('a'))
                .conflicts_with("exact")
                .help("Delete all matching Flights even when FLIGHT is ambiguous"))
    }
}

impl CliCommand for SeaplaneFlightDelete {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // TODO: find remote Flights too to check references

        // Get the indices of any flights that match the given name/ID
        let indices = if ctx.args.exact {
            ctx.db
                .flights
                .indices_of_matches(ctx.args.name_id.as_ref().unwrap())
        } else {
            ctx.db
                .flights
                .indices_of_left_matches(ctx.args.name_id.as_ref().unwrap())
        };

        match indices.len() {
            0 => errors::no_matching_item(ctx.args.name_id.clone().unwrap(), ctx.args.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !ctx.args.all {
                    errors::ambiguous_item(ctx.args.name_id.clone().unwrap(), true)?;
                }
            }
        }

        // Remove the flights
        ctx.db
            .flights
            .remove_indices(&indices)
            .iter()
            .for_each(|flight| {
                cli_println!("Deleted Flight {}", &flight.id.to_string());
            });

        ctx.persist_flights()?;

        cli_println!(
            "\nSuccessfully removed {} item{}",
            indices.len(),
            if indices.len() > 1 { "s" } else { "" }
        );

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.name_id = matches.value_of("flight").map(ToOwned::to_owned);
        Ok(())
    }
}
