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
            .about("Delete a local Flight Plan")
            .arg(arg!(flight required =["NAME|ID"])
                .validator(validator)
                .help("The name or ID of the Flight Plan to remove, must be unambiguous"))
            .arg(arg!(--force)
                .help("Delete this Flight Plan even if referenced by a local Formation Plan, or deletes ALL Flight Plan referenced by the name or ID even if ambiguous"))
            .arg(arg!(--all -('a'))
                .help("Delete all matching Flight Plans even when the name or ID is ambiguous"))
    }
}

impl CliCommand for SeaplaneFlightDelete {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.stateless {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("'");
            cli_eprint!(@Yellow, "--stateless");
            cli_eprint!("' cannot be used with '");
            cli_eprint!(@Yellow, "seaplane flight delete");
            cli_eprintln!("'");
            cli_eprintln!("(hint: 'seaplane flight delete' only modifies local plans)");
            cli_eprint!("(hint: you may want 'seaplane ");
            cli_eprint!(@Green, "formation ");
            cli_eprintln!("delete' instead)");
            std::process::exit(1);
        }

        // Get the indices of any flights that match the given name/ID
        let indices = if ctx.args.all {
            ctx.db
                .flights
                .indices_of_left_matches(ctx.args.name_id.as_ref().unwrap())
        } else {
            ctx.db
                .flights
                .indices_of_matches(ctx.args.name_id.as_ref().unwrap())
        };

        match indices.len() {
            0 => errors::no_matching_item(ctx.args.name_id.clone().unwrap(), false, ctx.args.all)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !ctx.args.all {
                    errors::ambiguous_item(ctx.args.name_id.clone().unwrap(), true)?;
                }
            }
        }

        // TODO: Check Formation References and require --force if found

        // Remove the flights
        ctx.db
            .flights
            .remove_indices(&indices)
            .iter()
            .for_each(|flight| {
                cli_println!("Deleted local Flight Plan {}", &flight.id.to_string());
            });

        ctx.persist_flights()?;

        if !ctx.internal_run {
            cli_println!(
                "\nSuccessfully removed {} item{}",
                indices.len(),
                if indices.len() > 1 { "s" } else { "" }
            );
        }
        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.name_id = matches.value_of("flight").map(ToOwned::to_owned);
        Ok(())
    }
}
