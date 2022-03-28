use clap::{ArgMatches, Command};

use crate::{
    cli::{
        cmds::metadata::{build_config_request_key, common},
        CliCommand,
    },
    context::{Ctx, MetadataCtx},
    error::Result,
    printer::{Output, OutputFormat},
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataGet;

impl SeaplaneMetadataGet {
    pub fn command() -> Command<'static> {
        // TODO: add a way to elide long keys or values with ... after a certain char count
        Command::new("get")
            .visible_alias("show")
            .override_usage("seaplane metadata get <KEY>... [OPTIONS]")
            .about("Get one or more metadata key-value pairs")
            .args(common::args())
            .args(common::display_args())
    }
}

impl CliCommand for SeaplaneMetadataGet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let kvs = {
            let mut mdctx = ctx.md_ctx();
            for kv in mdctx.kvs.iter_mut() {
                kv.set_value(
                    // The key is already in Base64 so no need to convert
                    build_config_request_key(kv.key.as_ref().unwrap().to_string(), ctx)?
                        .get_value()?
                        .to_string(),
                );
            }

            mdctx.kvs.clone()
        };
        match ctx.out_format {
            OutputFormat::Json => kvs.print_json(ctx)?,
            OutputFormat::Table => kvs.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.init_md(MetadataCtx::from_md_common(
            &common::SeaplaneMetadataCommonArgMatches(matches),
        )?);
        ctx.out_format = matches.value_of_t_or_exit("format");
        let mut mdctx = ctx.md_ctx();
        mdctx.decode = matches.is_present("decode");
        mdctx.disp_encoding = matches.value_of_t_or_exit("display-encoding");
        Ok(())
    }
}
