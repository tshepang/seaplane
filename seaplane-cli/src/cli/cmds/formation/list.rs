use std::fs;

use clap::Parser;

use crate::{
    error::Result,
    ops::formation::Formations,
    printer::{Output, Printer},
    Ctx, OutputFormat,
};

/// List your Seaplane Formations
#[derive(Parser)]
#[clap(visible_aliases = &["ls"])]
pub struct SeaplaneFormationListArgs {
    /// Change the output format
    #[clap(arg_enum, long, default_value = "table")]
    format: OutputFormat,
}

impl SeaplaneFormationListArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        Printer::init(ctx.color);

        let formations_file = ctx.formations_file();
        if !formations_file.exists() {
            // TODO: Inform the user nicely there is nothing to display and hint on what to do next
            return Ok(());
        }

        let json_str = fs::read_to_string(formations_file)?;
        let formations: Formations = serde_json::from_str(&json_str)?;

        // TODO: maybe don't hard code the endpoint
        // TODO: uncomment when we can make remote calls
        //let body = reqwest::blocking::get(ctx.compute_api_url("formations"))?.text()?;

        match ctx.out_format {
            OutputFormat::Json => formations.print_json(ctx)?,
            OutputFormat::Table => formations.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.out_format = self.format;

        Ok(())
    }
}
