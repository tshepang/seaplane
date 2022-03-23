use clap::{ArgMatches, Command};
use seaplane::api::v1::config::{Directory, Key, RangeQueryContext};

use crate::{
    cli::{
        cmds::kv::{build_config_request_dir, common},
        CliCommand,
    },
    context::{Ctx, KvCtx},
    error::Result,
    ops::kv::KeyValues,
    printer::{Output, OutputFormat},
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneKvList;

impl SeaplaneKvList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .override_usage("seaplane key-value list <DIR> [OPTIONS]")
            .about("List one or more key-value pairs")
            .arg(
                arg!(dir =["DIR"])
                    .help("The root directory of the key-value pairs to list"),
            )
            .arg(common::base64())
            .args(common::display_args())
            .arg(arg!(--after - ('a')).help("Only print key-value pairs after this key (note: this key and it's value are NOT included in the results)"))
    }
}

impl CliCommand for SeaplaneKvList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // Scope releases the mutex on the KvCtx so that when we hand off the ctx to print_* we
        // don't have the chance of a deadlock if those functions need to acquire a KvCtx
        let kvs = {
            let kvctx = ctx.kv_ctx();

            let mut range = RangeQueryContext::new();
            if let Some(dir) = &kvctx.directory {
                range.set_directory(dir.clone());
            }
            if let Some(after) = &kvctx.after {
                range.set_after(after.clone());
            }
            // Using the KeyValues container makes displaying easy
            KeyValues::from_model(build_config_request_dir(range, ctx)?.get_all_pages()?)
        };

        match ctx.out_format {
            OutputFormat::Json => kvs.print_json(ctx)?,
            OutputFormat::Table => kvs.print_table(ctx)?,
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
        kvctx.after = maybe_base64_arg!(matches, "after", matches.is_present("base64"))
            .map(Key::from_encoded);
        kvctx.directory = maybe_base64_arg!(matches, "dir", matches.is_present("base64"))
            .map(Directory::from_encoded);
        Ok(())
    }
}
