use clap::Parser;

use crate::{
    error::Result,
    fs::FromDisk,
    ops::flight::Flights,
    printer::{Output, Printer},
    Ctx, OutputFormat,
};

// TODO: add sorting
// TODO: add filtering
/// List the current Flight definitions
#[derive(Parser)]
#[clap(visible_aliases = &["ls"])]
pub struct SeaplaneFlightListArgs {
    /// Change the output format
    #[clap(arg_enum, long, default_value = "table")]
    format: OutputFormat,
}

impl SeaplaneFlightListArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        Printer::init(ctx.color);

        let flights: Flights = FromDisk::load(ctx.flights_file())?;

        // TOOD: get remote flights too

        match ctx.out_format {
            OutputFormat::Json => flights.print_json(ctx)?,
            OutputFormat::Table => flights.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        ctx.out_format = self.format;

        Ok(())
    }
}
