use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use hex::ToHex;
use rand::Rng;
use seaplane::api::v1::formations::Flight as FlightModel;
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;

use crate::{context::Ctx, printer::Output};

pub fn generate_name() -> String {
    // TODO: Maybe set an upper bound on the number of iterations and don't expect
    names::Generator::default()
        .find(|name| validate_name(name).is_ok())
        .expect("Failed to generate a random name")
}

// For now, somewhat arbitrary naming rules to ensure we don't go over the 63 char limit for
// hostnames in a URL (while considering how Seaplane combines the tenant name and expands `-`)
// Current Rules:
//  - may only include 0-9, a-z, A-Z, and '-' (hyphen)
//  - hyphens ('-') may not be repeated (i.e. '--')
//  - no more than three (3) total hyphens
//  - the total length must be <= 27
pub fn validate_name(name: &str) -> Result<(), String> {
    if name.len() > 27 {
        return Err("Flight name too long, must be <= 27 in length".into());
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("illegal character in Flight name".into());
    }
    if name.chars().filter(|c| *c == '-').count() > 3 {
        return Err("no more than three hyphens ('-') allowed in Flight name".into());
    }
    if name.contains("--") {
        return Err("repeated hyphens ('--') not allowed in Flight name".into());
    }

    Ok(())
}

// A wrapper round a Flight model
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Flight {
    #[serde(
        serialize_with = "hex::serde::serialize",
        deserialize_with = "hex::serde::deserialize"
    )]
    pub id: [u8; 32],
    #[serde(flatten)]
    pub model: FlightModel,
}

impl Flight {
    pub fn new(model: FlightModel) -> Self {
        Self {
            id: rand::thread_rng().gen(),
            model,
        }
    }

    pub fn starts_with(&self, s: &str) -> bool {
        hex::encode(self.id).starts_with(s) || self.model.name().starts_with(s)
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Flights {
    pub inner: Vec<Flight>,
}

impl Flights {
    /// Loads Flights from the given path or returns `Ok(None)` if path does not exist
    pub fn load_from_disk<P: AsRef<Path>>(p: P) -> Result<Option<Self>> {
        let path = p.as_ref();
        if !path.exists() {
            // TODO: Inform the user nicely there is nothing to display and hint on what to do next
            return Ok(None);
        }

        Ok(Some(serde_json::from_str(&fs::read_to_string(path)?)?))
    }

    // Returns true if a duplicate was found, but die=false.
    pub fn is_ambiguous_or_die(&self, needle: &str, die: bool) -> bool {
        // Check for duplicates and suggest `seaplane flight edit`
        if !self.indices_of_left_matches(needle).is_empty() {
            if die {
                cli_eprint!(@Red, "error: ");
                cli_eprint!("a Flight with the name '");
                cli_eprint!(@Yellow, "{}", needle);
                cli_eprintln!("' already exists");
                cli_eprint!("(hint: try '");
                cli_eprint!(@Green, "seaplane flight edit {}", needle);
                cli_eprintln!("' instead)");

                std::process::exit(1);
            }
            return true;
        }
        false
    }

    /// Removes any Flight definitions from the matching indices and returns a Vec of all removed
    /// Flights
    pub fn remove_indices(&mut self, indices: &[usize]) -> Vec<Flight> {
        // TODO: There is probably a much more performant way to remove a bunch of times from a Vec
        // but we're talking such a small number of items this should never matter.

        indices
            .iter()
            .enumerate()
            .map(|(i, idx)| self.inner.remove(idx - i))
            .collect()
    }

    /// Returns all indices of where the flight's Name or ID (as hex) begins with the `needle`
    pub fn indices_of_left_matches(&self, needle: &str) -> Vec<usize> {
        // TODO: Are there any names that also coincide with valid hex?
        self.inner
            .iter()
            .enumerate()
            .filter(|(_idx, flight)| flight.starts_with(needle))
            .map(|(idx, _flight)| idx)
            .collect()
    }

    /// Returns all indices of where the flight's Name or ID (as hex) is an exact match of `needle`
    pub fn indices_of_matches(&self, needle: &str) -> Vec<usize> {
        // TODO: Are there any names that also coincide with valid hex?
        self.inner
            .iter()
            .enumerate()
            .filter(|(_idx, flight)| {
                hex::encode(flight.id) == needle || flight.model.name() == needle
            })
            .map(|(idx, _flight)| idx)
            .collect()
    }

    /// Serializes itself to the given path
    pub fn save_to_disk<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        // TODO: make atomic so that we don't lose or currupt data
        // TODO: long term consider something like SQLite
        serde_json::to_writer(File::create(p)?, self).map_err(Into::into)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Flight> {
        self.inner.iter()
    }
}

impl Output for Flights {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);

        Ok(())
    }

    fn print_table(&self, _ctx: &Ctx) -> Result<()> {
        let buf = Vec::new();
        let mut tw = TabWriter::new(buf);
        // TODO: Add local/remote formation references
        writeln!(tw, "LOCAL ID\tNAME\tIMAGE\tMIN\tMAX\tARCH\tAPI PERMS")?;
        for flight in self.iter() {
            let arch = flight
                .model
                .architecture()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            writeln!(
                tw,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                &flight.id.encode_hex::<String>()[..8], // TODO: make sure length is not ambiguous
                flight.model.name(),
                flight
                    .model
                    .image_str()
                    .trim_start_matches("registry.seaplanet.io/"), // TODO: provide opt-in/out way to collapse long names
                flight.model.minimum(),
                flight
                    .model
                    .maximum()
                    .map(|n| format!("{n}"))
                    .unwrap_or_else(|| "INF".into()),
                if arch.is_empty() { "auto" } else { &*arch },
                flight.model.api_permission(),
            )?;
        }
        tw.flush()?;

        cli_println!("{}", String::from_utf8_lossy(&tw.into_inner()?));

        Ok(())
    }
}
