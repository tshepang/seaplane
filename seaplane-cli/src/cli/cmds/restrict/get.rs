use clap::{ArgMatches, Command};

use crate::{
    api::RestrictReq,
    cli::{cmds::restrict::common, CliCommand},
    context::{Ctx, RestrictCtx},
    error::Result,
    printer::{Output, OutputFormat},
};

static LONG_ABOUT: &str = "Get information about restrictions on a directory

Directory will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Use --decode to output the decoded values instead.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneRestrictGet;

impl SeaplaneRestrictGet {
    pub fn command() -> Command {
        Command::new("get")
            .visible_alias("show")
            .about("Retrieve information about a directory restriction")
            .long_about(LONG_ABOUT)
            .arg(common::api())
            .arg(common::directory())
            .arg(common::base64())
            .args(common::display_args())
    }
}

impl CliCommand for SeaplaneRestrictGet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let restriction = {
            let mut req = RestrictReq::new(ctx)?;
            let restrict_ctx = ctx.restrict_ctx.get_mut_or_init();
            req.set_api(restrict_ctx.api.as_ref().unwrap())?;
            req.set_directory(restrict_ctx.directory.as_ref().unwrap().to_string())?;
            req.get_restriction()?
        };
        match ctx.args.out_format {
            OutputFormat::Json => restriction.print_json(ctx)?,
            OutputFormat::Table => restriction.print_table(ctx)?,
        }
        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.restrict_ctx.init(RestrictCtx::from_restrict_common(
            &common::SeaplaneRestrictCommonArgMatches(matches),
        )?);
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut restrict_ctx = ctx.restrict_ctx.get_mut_or_init();
        restrict_ctx.decode = matches.get_flag("decode");
        restrict_ctx.no_header = matches.get_flag("no-header");
        Ok(())
    }

    fn next_subcmd<'a>(
        &self,
        _matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        None
    }
}
