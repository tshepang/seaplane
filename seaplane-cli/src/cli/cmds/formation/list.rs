use clap::{ArgMatches, Command};
use strum::VariantNames;

use crate::{cli::CliCommand, error::Result, printer::Output, Ctx, OutputFormat};

static LONG_ABOUT: &str = "List your Seaplane Formations

This command will display the status and number of configurations for each of your Formations.
The Formations displayed come from the local database of know Formations. You may wish to update
the local database with Remote Formations as well by first running:

$ seaplane formation fetch-remote

After which your local database will contain all remote Formations and their configurations as well.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationList;

impl SeaplaneFormationList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .long_about(LONG_ABOUT)
            .about("List your Seaplane Formations")
            .arg(
                arg!(--format =["FORMAT"=>"table"])
                    .possible_values(OutputFormat::VARIANTS)
                    .help("Change the output format"),
            )
    }
}

impl CliCommand for SeaplaneFormationList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        match ctx.args.out_format {
            OutputFormat::Json => ctx.db.formations.print_json(ctx)?,
            OutputFormat::Table => ctx.db.formations.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.out_format = matches.value_of_t("format").unwrap_or_default();
        Ok(())
    }
}
