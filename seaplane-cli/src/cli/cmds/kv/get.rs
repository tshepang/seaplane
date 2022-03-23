use clap::{ArgMatches, Command};

use crate::{
    cli::{
        cmds::kv::{build_config_request_key, common},
        CliCommand,
    },
    context::{Ctx, KvCtx},
    error::Result,
    printer::{Output, OutputFormat},
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneKvGet;

impl SeaplaneKvGet {
    pub fn command() -> Command<'static> {
        // TODO: add a way to elide long keys or values with ... after a certain char count
        Command::new("get")
            .visible_alias("show")
            .override_usage("seaplane key-value get <KEY>... [OPTIONS]")
            .about("Get one or more key-value pairs")
            .args(common::args())
            .args(common::display_args())
    }
}

impl CliCommand for SeaplaneKvGet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        for kv in ctx.kv_ctx().kvs.iter_mut() {
            kv.set_value(
                // The key is already in Base64 so no need to convert
                build_config_request_key(kv.key.as_ref().unwrap().to_string(), ctx)?
                    .get_value()?
                    .to_string(),
            );
        }

        match ctx.out_format {
            OutputFormat::Json => ctx.kv_ctx().kvs.print_json(ctx)?,
            OutputFormat::Table => ctx.kv_ctx().kvs.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.init_kv(KvCtx::from_kv_common(&common::SeaplaneKvCommonArgMatches(
            matches,
        ))?);
        let mut kvctx = ctx.kv_ctx();
        kvctx.decode = matches.is_present("decode");
        kvctx.disp_encoding = matches.value_of_t_or_exit("display-encoding");
        Ok(())
    }
}
