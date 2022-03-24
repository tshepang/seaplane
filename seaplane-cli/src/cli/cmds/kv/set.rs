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

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneKvSetArgMatches<'a>(pub &'a ArgMatches);

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneKvSet;

impl SeaplaneKvSet {
    pub fn command() -> Command<'static> {
        Command::new("set")
            .visible_alias("put")
            .override_usage("seaplane key-value set <KEY:VALUE> [OPTIONS]")
            .about("Set a key-value pair")
            .arg(common::base64())
            .arg(arg!(key =["KEY"] required ).help("The key to set"))
            .arg(arg!(value =["VALUE"] required ).help("The value (@path will load the value from a path and @- will load the value from STDIN)"))
    }
}

impl CliCommand for SeaplaneKvSet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        for kv in ctx.kv_ctx().kvs.iter_mut() {
            let key = kv.key.as_ref().unwrap().to_string();
            let value = kv.value.as_ref().unwrap().to_string();
            build_config_request_key(&key, ctx)?.put_value(&value)?;
            if ctx.out_format == OutputFormat::Table {
                cli_println!("Set {key} with value {value}");
            }
        }

        if ctx.out_format == OutputFormat::Json {
            // Scope to release the KvCtx lock
            let kvs = { ctx.kv_ctx().kvs.clone() };
            kvs.print_json(ctx)?;
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.init_kv(KvCtx::from_kv_set(&SeaplaneKvSetArgMatches(matches))?);
        ctx.out_format = matches.value_of_t_or_exit("format");
        Ok(())
    }
}
