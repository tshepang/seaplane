use anyhow::Result;
use clap::Parser;
use hex::ToHex;

use crate::{data::flight::Flights, printer::Printer, Ctx};

/// Delete a Flight definition
#[derive(Parser)]
#[clap(visible_aliases = &["del", "remove", "rm"], override_usage = "seaplane flight delete <NAME|ID> [OPTIONS]")]
pub struct SeaplaneFlightDeleteArgs {
    /// The name or hash of the Flight to remove, must be unambiguous
    #[clap(value_name = "NAME|ID")]
    flight: String,

    /// Delete this Flight even if referenced by a Formation (removes any references in
    /// Formations), or deletes ALL Flights referencedd by <FLIGHT> even if ambiguous
    #[clap(long)]
    force: bool,

    /// the given FLIGHT must be an exact match
    #[clap(short = 'x', long)]
    exact: bool,

    /// Delete all matching Flights even when FLIGHT is ambiguous
    #[clap(short, long)]
    all: bool,
    // TODO: add a --local[-only] flag or similar that combines with --force to only remove local
}

impl SeaplaneFlightDeleteArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        Printer::init(ctx.color);

        // Load the known Flights from the local JSON "DB"
        let mut flights = Flights::load_from_disk(ctx.flights_file())?.unwrap_or_default();

        // TODO: find remote Flights too to check references

        // Get the indices of any flights that match the given name/ID
        let indices = if self.exact {
            flights.indices_of_matches(&self.flight)
        } else {
            flights.indices_of_left_matches(&self.flight)
        };

        match indices.len() {
            0 => {
                cli_eprint!(@Red, "error: ");
                cli_eprint!("the NAME or ID '");
                cli_eprint!(@Green, "{}", self.flight);
                cli_eprintln!("' didn't match anything");
                std::process::exit(1);
            }
            1 => (),
            _ => {
                // TODO: and --force
                if !self.all {
                    cli_eprint!(@Red, "error: ");
                    cli_eprint!("the name or hash '");
                    cli_eprint!(@Yellow, "{}", self.flight);
                    cli_eprintln!("' is ambiguous and matches more than one item");
                    cli_eprint!("(hint: try adding '");
                    cli_eprint!(@Green, "--all");
                    cli_eprintln!("' to remove all matching items)");
                    std::process::exit(1);
                }
            }
        }

        // Remove the flights
        flights.remove_indices(&indices).iter().for_each(|flight| {
            cli_println!("Deleted Flight {}", &flight.id.encode_hex::<String>());
        });

        // Write out an entirely new JSON file with the Flight(s) deleted
        flights.save_to_disk(ctx.flights_file())?;

        cli_println!(
            "\nSuccessfully removed {} item{}",
            indices.len(),
            if indices.len() > 1 { "s" } else { "" }
        );

        Ok(())
    }
}
