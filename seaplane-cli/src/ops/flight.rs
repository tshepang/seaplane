use std::{
    io::Write,
    path::{Path, PathBuf},
};

use seaplane::api::{v1::formations::Flight as FlightModel, IMAGE_REGISTRY_URL};
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;

use crate::{
    context::{Ctx, FlightCtx},
    error::{CliError, CliErrorKind, Result},
    fs::{FromDisk, ToDisk},
    ops::Id,
    printer::Output,
};

/// A wrapper round a Flight model
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Flight {
    pub id: Id,
    #[serde(flatten)]
    pub model: FlightModel,
}

impl Flight {
    pub fn new(model: FlightModel) -> Self {
        Self {
            id: Id::new(),
            model,
        }
    }

    pub fn from_json(s: &str) -> Result<Flight> {
        serde_json::from_str(s).map_err(CliError::from)
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.id.to_string().starts_with(s) || self.model.name().starts_with(s)
    }

    /// Applies the non-default differences from `ctx`
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

impl FromDisk for Flights {
    fn set_loaded_from<P: AsRef<Path>>(&mut self, p: P) {
        self.loaded_from = Some(p.as_ref().into());
    }

    fn loaded_from(&self) -> Option<&Path> {
        self.loaded_from.as_deref()
    }
}

impl ToDisk for Flights {}

impl Flights {
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
                flight.id.to_string() == needle || flight.model.name() == needle
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

    pub fn update_or_create_flight(&mut self, model: FlightModel) -> Vec<(String, Id)> {
        let found = false;
        let mut ret = Vec::new();
        for flight in self
            .inner
            .iter_mut()
            .filter(|f| f.model.name() == model.name() && f.model.image_str() == model.image_str())
        {
            flight.model.set_minimum(model.minimum());
            flight.model.set_maximum(model.maximum());

            for arch in model.architecture() {
                flight.model.add_architecture(*arch);
            }

            flight.model.set_api_permission(model.api_permission());

            ret.push((flight.model.name().to_owned(), flight.id));
        }

        if !found {
            let f = Flight::new(model);
            ret.push((f.model.name().to_owned(), f.id));
            self.inner.push(f);
        }

        ret
    }

    pub fn update_flight(&mut self, src: &str, exact: bool, ctx: &FlightCtx) -> Result<()> {
        let mut src_flight = self.remove_flight(src, exact)?;
        src_flight.update_from(ctx)?;

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

    pub fn find_name(&self, name: &str) -> Option<&Flight> {
        self.inner.iter().find(|f| f.model.name() == name)
    }

    pub fn find_name_or_partial_id(&self, needle: &str) -> Option<&Flight> {
        self.inner
            .iter()
            .find(|f| f.model.name() == needle || f.id.to_string().starts_with(needle))
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
                &flight.id.to_string()[..8], // TODO: make sure length is not ambiguous
                flight.model.name(),
                flight
                    .model
                    .image_str()
                    .trim_start_matches(IMAGE_REGISTRY_URL), // TODO: provide opt-in/out way to collapse long names
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
