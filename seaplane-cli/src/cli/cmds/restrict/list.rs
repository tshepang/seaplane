use clap::{ArgMatches, Command};

use crate::{
    api::RestrictReq,
    cli::{cmds::restrict::common, CliCommand},
    context::{Ctx, RestrictCtx},
    error::Result,
    ops::restrict::Restrictions,
    printer::{Output, OutputFormat},
};

static LONG_ABOUT: &str = "List restrictions in an API, or across all APIs

Directory will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Use --decode to output the decoded values instead.";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneRestrictListArgMatches<'a>(pub &'a ArgMatches);

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneRestrictList;

impl SeaplaneRestrictList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .override_usage("seaplane restrict list [API] [OPTIONS]")
            .about("List restrictions in an API, or across all APIs")
            .long_about(LONG_ABOUT)
            .arg(arg!(api = ["API"]).help("The API to list the restrictions from"))
            .arg(common::base64())
            .args(common::display_args())
    }
}

impl CliCommand for SeaplaneRestrictList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let restrictions = {
            let mut req = RestrictReq::new(ctx)?;
            let restrict_ctx = ctx.restrict_ctx.get_mut_or_init();
            restrict_ctx.api.as_ref().map(|api| req.set_api(api));

            Restrictions::from_model(req.get_all_pages()?)
        };

        match ctx.args.out_format {
            OutputFormat::Json => restrictions.print_json(ctx)?,
            OutputFormat::Table => restrictions.print_table(ctx)?,
        }
        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.restrict_ctx
            .init(RestrictCtx::from_restrict_list(&SeaplaneRestrictListArgMatches(matches))?);
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut restrict_ctx = ctx.restrict_ctx.get_mut_or_init();
        restrict_ctx.decode = matches.contains_id("decode");
        restrict_ctx.no_header = matches.contains_id("no-header");
        Ok(())
    }

    fn next_subcmd<'a>(
        &self,
        _matches: &'a ArgMatches,
    ) -> Option<(Box<dyn CliCommand>, &'a ArgMatches)> {
        None
    }
}
