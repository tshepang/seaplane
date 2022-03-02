use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    result::Result as StdResult,
};

use hex::ToHex;
use rand::Rng;
use seaplane::api::v1::formations::Flight as FlightModel;
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;

use crate::{
    context::{Ctx, FlightCtx},
    error::{CliError, CliErrorKind, Context, Result},
    printer::{Color, Output},
};

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
pub fn validate_name(name: &str) -> StdResult<(), String> {
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

    // Applies the non-default differences from `ctx`
    pub fn update_from(&mut self, ctx: &FlightCtx) -> Result<()> {
        let mut dest_builder = FlightModel::builder();

        // Name
        dest_builder = dest_builder.name(&ctx.name);

        if let Some(image) = ctx.image.clone() {
            dest_builder = dest_builder.image_reference(image);
        } else {
            dest_builder = dest_builder.image_reference(self.model.image().clone());
        }

        if ctx.minimum != 1 {
            dest_builder = dest_builder.minimum(ctx.minimum);
        } else {
            dest_builder = dest_builder.minimum(self.model.minimum());
        }

        if let Some(max) = ctx.maximum {
            dest_builder = dest_builder.maximum(max);
        } else if ctx.reset_maximum {
            dest_builder.clear_maximum();
        } else if let Some(max) = self.model.maximum() {
            dest_builder = dest_builder.maximum(max);
        }

        // Architecture
        for arch in ctx.architecture.iter().chain(self.model.architecture()) {
            dest_builder = dest_builder.add_architecture(*arch);
        }

        // API Permission
        let orig_api_perms = self.model.api_permission();
        let cli_api_perms = ctx.api_permission;
        match (orig_api_perms, cli_api_perms) {
            (true, false) => dest_builder = dest_builder.api_permission(false),
            (false, true) => dest_builder = dest_builder.api_permission(true),
            _ => (),
        }

        self.model = dest_builder.build().expect("Failed to build Flight");
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Flights {
    #[serde(skip)]
    loaded_from: Option<PathBuf>,
    pub inner: Vec<Flight>,
}

impl Flights {
    /// Loads Flights from the given path or returns `Ok(None)` if path does not exist
    pub fn load_from_disk<P: AsRef<Path>>(p: P) -> Result<Self> {
        let path = p.as_ref();

        let mut flights: Flights = serde_json::from_str(
            &fs::read_to_string(&path)
                .map_err(CliError::from)
                .context("\n\tpath: ")
                .with_color_context(|| (Color::Yellow, format!("{:?}\n", path)))
                .context("\n(hint: try '")
                .color_context(Color::Green, "seaplane init")
                .context("' if the files are missing)\n")?,
        )?;
        flights.loaded_from = Some(path.into());

        Ok(flights)
    }

    /// Serializes itself to the given path
    pub fn save_to_disk(&self) -> Result<()> {
        let path = &self
            .loaded_from
            .as_ref()
            .expect("Flights created without manually (de)serializing 'loaded_from'");

        // TODO: make atomic so that we don't lose or currupt data
        // TODO: long term consider something like SQLite
        serde_json::to_writer(
            File::create(path)
                .map_err(CliError::from)
                .context("\n\tpath: ")
                .with_color_context(|| (Color::Yellow, format!("{:?}\n", path)))?,
            self,
        )
        .map_err(CliError::from)
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

    pub fn iter(&self) -> impl Iterator<Item = &Flight> {
        self.inner.iter()
    }

    pub fn clone_flight(&mut self, src: &str, exact: bool) -> Result<Flight> {
        let src_flight = self.remove_flight(src, exact)?;
        let model = src_flight.model.clone();

        // Re add the source flight
        self.inner.push(src_flight);

        Ok(Flight::new(model))
    }

    pub fn update_flight(&mut self, src: &str, exact: bool, ctx: &FlightCtx) -> Result<()> {
        let mut src_flight = self.remove_flight(src, exact)?;
        src_flight.update_from(ctx);

        // Re add the source flight
        self.inner.push(src_flight);

        Ok(())
    }

    pub fn remove_flight(&mut self, src: &str, exact: bool) -> Result<Flight> {
        // Ensure only one current Flight matches what the user gave
        // TODO: We should check if these ones we remove are referenced remote or not
        let indices = if exact {
            self.indices_of_matches(src)
        } else {
            self.indices_of_left_matches(src)
        };
        match indices.len() {
            0 => return Err(CliErrorKind::NoMatchingItem(src.into()).into_err()),
            1 => (),
            _ => return Err(CliErrorKind::AmbiguousItem(src.into()).into_err()),
        }

        Ok(self.remove_indices(&indices).pop().unwrap())
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

        cli_println!(
            "{}",
            String::from_utf8_lossy(
                &tw.into_inner()
                    .map_err(|_| CliError::bail("IO flush error"))?
            )
        );

        Ok(())
    }
}
