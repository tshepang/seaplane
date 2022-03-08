use clap::Parser;

use crate::{
    error::Result, fs::FromDisk, ops::formation::Formations, printer::Output, Ctx, OutputFormat,
};

/// List your Seaplane Formations
#[derive(Parser)]
#[clap(visible_aliases = &["ls"], long_about = "List your Seaplane Formations

This command will display the status and number of configurations for each of your Formations.
The Formations displayed come from the local database of know Formations. You may wish to update
the local database with Remote Formations as well by first running:

$ seaplane formation fetch-remote

After which your local database will contain all remote Formations and their configurations as well.")]
pub struct SeaplaneFormationListArgs {
    /// Change the output format
    #[clap(arg_enum, long, default_value = "table")]
    format: OutputFormat,
}

impl SeaplaneFormationListArgs {
    pub fn run(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;

        let formations: Formations = FromDisk::load(ctx.formations_file())?;

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
