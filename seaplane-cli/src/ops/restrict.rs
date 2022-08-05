use std::{fmt::Display, io::Write};

use seaplane::api::v1::restrict::Restriction;
use tabwriter::TabWriter;

use crate::{context::Ctx, error::Result, printer::Output};

impl Output for Restriction {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let show_headers = !ctx.locks_ctx.get_or_init().no_header;
        let mut tw = TabWriter::new(Vec::new());

        if show_headers {
            writeln!(tw, "API\tDIRECTORY\tSTATE\tREGIONS ALLOWED\tREGIONS DENIED\tPROVIDERS ALLOWED\tPROVIDERS DENIED")?;
        }

        // Helper function for displaying region and provider vectors
        fn join_vector<S: Display>(vector: Vec<S>) -> String {
            vector
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        }

        writeln!(
            tw,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.api,
            self.directory,
            self.state,
            join_vector(self.details.regions_allowed.clone()),
            join_vector(self.details.regions_denied.clone()),
            join_vector(self.details.providers_allowed.clone()),
            join_vector(self.details.providers_denied.clone())
        )?;
        tw.flush()?;

        Ok(())
    }
}
