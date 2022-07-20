use clap::{value_parser, ArgMatches, Command};

use crate::{
    cli::{cmds::formation::SeaplaneFormationFetch, CliCommand},
    error::Result,
    printer::Output,
    Ctx, OutputFormat,
};

static LONG_ABOUT: &str = "List all local Formation Plans

This command will display the status and number of configurations for each of your Formation
Plans. The Formations displayed come from the local database of known Formations. You may wish
to update the local database with Remote Formation Instances as well by either first running:

$ seaplane formation fetch-remote

OR including `--fetch` such as:

$ seaplane formation list --fetch

After which your local database of Formation and Flight Plans will contain all remote Formation
Instances and their configurations as well.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationList;

impl SeaplaneFormationList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .long_about(LONG_ABOUT)
            .about("List all local Formation Plans")
            .arg(arg!(--fetch|sync|synchronize - ('F')).help("Fetch remote Formation Instances and create/synchronize with local Plan Definitions prior to listing (by default only local Plans are displayed)"))
            .arg(
                arg!(--format =["FORMAT"=>"table"])
                    .value_parser(value_parser!(OutputFormat))
                    .help("Change the output format"),
            )
    }
}

impl CliCommand for SeaplaneFormationList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        if ctx.args.stateless && !ctx.args.fetch {
            cli_eprint!(@Red, "error: ");
            cli_eprint!("'");
            cli_eprint!(@Yellow, "seaplane formation list");
            cli_eprint!("' when used with '");
            cli_eprint!(@Yellow, "--stateless");
            cli_eprint!("' is useless without '");
            cli_eprint!(@Green, "--fetch");
            cli_eprintln!("'");
            cli_eprintln!("(hint: 'seaplane formation list' only looks at local Plan definitions)");
            cli_eprint!("(hint: 'seaplane formation list");
            cli_eprint!(@Green, "--fetch");
            cli_eprintln!("' also synchronizes local Plan definitions with remote Instances)");
            std::process::exit(1);
        }

        if ctx.args.fetch {
            let old_name = ctx.args.name_id.take();
            SeaplaneFormationFetch.run(ctx)?;
            ctx.args.name_id = old_name;
        }

        match ctx.args.out_format {
            OutputFormat::Json => ctx.db.formations.print_json(ctx)?,
            OutputFormat::Table => ctx.db.formations.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        ctx.args.fetch = matches.contains_id("fetch");
        Ok(())
    }
}
