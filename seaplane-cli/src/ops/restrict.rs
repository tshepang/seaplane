use std::{collections::BTreeSet, fmt::Display, io::Write};

use seaplane::api::v1::restrict::{RestrictedDirectory as RestrictedDirectoryModel, Restriction};
use serde::Serialize;
use tabwriter::TabWriter;

use crate::{
    context::Ctx,
    error::{CliError, Result},
    printer::{printer, Output},
};

use super::EncodedString;

/// We use our own RestrictedDirectory instead of the models because we need to
/// *not* enforce base64 encoding, and implement a bunch of additional methods
/// and traits that wouldn't make sense for the models
///
/// We also need to keep track if the values are encoded or not
#[derive(Debug, Clone, Serialize)]
pub struct RestrictedDirectory {
    pub directory: EncodedString,
}

impl RestrictedDirectory {
    /// Creates a new RestrictedDirectory from an encoded directory.
    /// The directory must be URL safe base64 encoded or Bad Things may happen.
    pub fn new<S: Into<String>>(directory: S) -> Self {
        Self {
            directory: EncodedString::new(directory.into()),
        }
    }

    /// Creates a new RestrictedDirectoryModel from self's data.
    pub fn to_model(&self) -> RestrictedDirectoryModel {
        RestrictedDirectoryModel::from_encoded(self.directory.to_string())
    }
}

impl Output for Restriction {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let restrict_ctx = ctx.restrict_ctx.get_or_init();
        let restrictions = Restrictions::from_model(vec![self.clone()]);
        restrictions.impl_print_table(!restrict_ctx.no_header, restrict_ctx.decode)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(transparent)]
pub struct Restrictions {
    inner: Vec<Restriction>,
}

impl Restrictions {
    pub fn from_model(model: Vec<Restriction>) -> Self {
        Self { inner: model }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Restriction> {
        self.inner.iter()
    }

    // print a table in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_table(&self, headers: bool, decode: bool) -> Result<()> {
        let mut tw = TabWriter::new(Vec::new());

        // Helper function for displaying region and provider HashSets
        fn join_set<S: Display>(set: BTreeSet<S>) -> String {
            let mut vec = set.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            vec.sort();
            vec.join(",")
        }

        if headers {
            writeln!(tw, "API\tDIRECTORY\tSTATE\tREGIONS ALLOWED\tREGIONS DENIED\tPROVIDERS ALLOWED\tPROVIDERS DENIED")?;
        }

        for restriction in self.iter() {
            let rd = RestrictedDirectory::new(restriction.directory.encoded());
            write!(tw, "{}\t", restriction.api)?;

            if decode {
                tw.write_all(&rd.directory.decoded()?)?;
            } else {
                write!(tw, "{}", rd.directory)?;
            }

            writeln!(
                tw,
                "\t{}\t[{}]\t[{}]\t[{}]\t[{}]",
                restriction.state,
                join_set(restriction.details.regions_allowed.clone()),
                join_set(restriction.details.regions_denied.clone()),
                join_set(restriction.details.providers_allowed.clone()),
                join_set(restriction.details.providers_denied.clone())
            )?;
        }
        tw.flush()?;

        let mut ptr = printer();
        let page = tw
            .into_inner()
            .map_err(|_| CliError::bail("IO flush error writing restrictions"))?;
        ptr.write_all(&page)?;
        ptr.flush()?;

        Ok(())
    }
}

impl Output for Restrictions {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let restrict_ctx = ctx.restrict_ctx.get_or_init();
        self.impl_print_table(!restrict_ctx.no_header, restrict_ctx.decode)
    }
}
