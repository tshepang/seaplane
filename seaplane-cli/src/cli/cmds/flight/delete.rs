use clap::Parser;
use hex::ToHex;

use crate::{
    cli::errors,
    error::Result,
    fs::{FromDisk, ToDisk},
    ops::flight::Flights,
    printer::Printer,
    Ctx,
};

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
        let flights_file = ctx.flights_file();
        let mut flights: Flights = FromDisk::load(&flights_file).unwrap_or_default();

        // TODO: find remote Flights too to check references

        // Get the indices of any flights that match the given name/ID
        let indices = if self.exact {
            flights.indices_of_matches(&self.flight)
        } else {
            flights.indices_of_left_matches(&self.flight)
        };

        match indices.len() {
            0 => errors::no_matching_item(self.flight.clone(), self.exact)?,
            1 => (),
            _ => {
                // TODO: and --force
                if !self.all {
                    errors::ambiguous_item(self.flight.clone(), true)?;
                }
            }
        }

        // Remove the flights
        flights.remove_indices(&indices).iter().for_each(|flight| {
            cli_println!("Deleted Flight {}", &flight.id.encode_hex::<String>());
        });

        // Write out an entirely new JSON file with the Flight(s) deleted
        flights.persist()?;

        cli_println!(
            "\nSuccessfully removed {} item{}",
            indices.len(),
            if indices.len() > 1 { "s" } else { "" }
        );

        Ok(())
    }
}
