use clap::{ArgMatches, Command};

use seaplane::api::v1::config::Value;

use crate::{
    cli::{
        cmds::metadata::{build_config_request_key, common},
        CliCommand,
    },
    context::{Ctx, MetadataCtx},
    error::Result,
    printer::{Output, OutputFormat},
};

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneMetadataSetArgMatches<'a>(pub &'a ArgMatches);

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataSet;

impl SeaplaneMetadataSet {
    pub fn command() -> Command<'static> {
        Command::new("set")
            .visible_alias("put")
            .override_usage("seaplane metadata set <KEY:VALUE> [OPTIONS]")
            .about("Set a metadata key-value pair")
            .arg(common::base64())
            .arg(arg!(key =["KEY"] required ).help("The key to set"))
            .arg(arg!(value =["VALUE"] required ).help("The value (@path will load the value from a path and @- will load the value from STDIN)"))
    }
}

impl CliCommand for SeaplaneMetadataSet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        for kv in ctx.md_ctx().kvs.iter_mut() {
            let key = kv.key.as_ref().unwrap().to_string();
            let value = kv.value.as_ref().unwrap().to_string();
            build_config_request_key(&key, ctx.args.api_key()?)?
                .put_value(Value::from_encoded(value.clone()))?;
            if ctx.args.out_format == OutputFormat::Table {
                cli_println!("Set {key} with value {value}");
            }
        }

        if ctx.args.out_format == OutputFormat::Json {
            // Scope to release the MetadataCtx lock
            let kvs = { ctx.md_ctx().kvs.clone() };
            kvs.print_json(ctx)?;
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.init_md(MetadataCtx::from_md_set(&SeaplaneMetadataSetArgMatches(
            matches,
        ))?);
        ctx.args.out_format = matches.value_of_t_or_exit("format");
        Ok(())
    }
}
