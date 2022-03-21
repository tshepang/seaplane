use clap::{ArgMatches, Command};
use strum::VariantNames;

use crate::{
    cli::CliCommand, error::Result, fs::FromDisk, ops::flight::Flights, printer::Output, Ctx,
    OutputFormat,
};

// TODO: add sorting
// TODO: add filtering
#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightList;

impl SeaplaneFlightList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .about("List the current Flight definitions")
            .arg(
                arg!(--format =["FORMAT"=>"table"])
                    .help("Change the output format")
                    .possible_values(OutputFormat::VARIANTS),
            )
    }
}

impl CliCommand for SeaplaneFlightList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let flights: Flights = FromDisk::load(ctx.flights_file())?;

        // TODO: get remote flights too

        match ctx.out_format {
            OutputFormat::Json => flights.print_json(ctx)?,
            OutputFormat::Table => flights.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.out_format = matches.value_of_t("format").unwrap_or_default();
        Ok(())
    }
}
