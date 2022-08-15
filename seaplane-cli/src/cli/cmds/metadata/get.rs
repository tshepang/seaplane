use clap::{ArgMatches, Command};

use crate::{
    api::MetadataReq,
    cli::{cmds::metadata::common, CliCommand},
    context::{Ctx, MetadataCtx},
    error::Result,
    printer::{Output, OutputFormat},
};

static LONG_ABOUT: &str = "Retrieve a metadata key-value pair

Keys and values will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Use --decode to output the decoded values instead.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataGet;

impl SeaplaneMetadataGet {
    pub fn command() -> Command<'static> {
        // TODO: add a way to elide long keys or values with ... after a certain char count
        Command::new("get")
            .visible_alias("show")
            .override_usage("seaplane metadata get <KEY> [OPTIONS]")
            .about("Retrieve a metadata key-value pair")
            .long_about(LONG_ABOUT)
            .arg(common::single_key())
            .arg(common::base64())
            .args(common::display_args())
            .mut_arg("no-header", |a| a.hide(true))
            .mut_arg("only-keys", |a| a.hide(true))
            .mut_arg("only-values", |a| a.hide(true))
    }
}

impl CliCommand for SeaplaneMetadataGet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let kvs = {
            let mut req = MetadataReq::new(ctx)?;
            let mdctx = ctx.md_ctx.get_mut_or_init();
            for kv in mdctx.kvs.iter_mut() {
                req.set_key(kv.key.as_ref().unwrap().to_string())?;
                kv.set_value(
                    // The key is already in Base64 so no need to convert
                    req.get_value()?.to_string(),
                );
            }

            mdctx.kvs.clone()
        };
        match ctx.args.out_format {
            OutputFormat::Json => kvs.print_json(ctx)?,
            OutputFormat::Table => kvs.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.md_ctx
            .init(MetadataCtx::from_md_common(&common::SeaplaneMetadataCommonArgMatches(matches))?);
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut mdctx = ctx.md_ctx.get_mut_or_init();
        mdctx.decode = matches.contains_id("decode");
        mdctx.no_header = true;
        mdctx.no_keys = true;
        mdctx.no_values = false;
        Ok(())
    }
}
