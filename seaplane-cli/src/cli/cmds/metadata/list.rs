use clap::{ArgMatches, Command};
use seaplane::api::v1::{config::Key, Directory, RangeQueryContext};

use crate::{
    api::ConfigReq,
    cli::{cmds::metadata::common, CliCommand},
    context::{Ctx, MetadataCtx},
    error::Result,
    ops::metadata::KeyValues,
    printer::{Output, OutputFormat},
};

static LONG_ABOUT: &str = "List one or more metadata key-value pairs

Keys and values will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values. However since they may contain arbitrary data, it's possible to re-encode them into a
different format for display purposes using --display-encoding";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataList;

impl SeaplaneMetadataList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .override_usage("seaplane metadata list <DIR> [OPTIONS]")
            .about("List one or more metadata key-value pairs")
            .long_about(LONG_ABOUT)
            .arg(
                arg!(dir =["DIR"])
                    .help("The root directory of the metadata key-value pairs to list"),
            )
            .arg(common::base64())
            .args(common::display_args())
            .arg(arg!(--from - ('f') =["KEY"]).help("Only print metadata key-value pairs after this key (note: if this key has a value it will be included in the results)"))
    }
}

impl CliCommand for SeaplaneMetadataList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        // Scope releases the mutex on the MetadataCtx so that when we hand off the ctx to print_* we
        // don't have the chance of a deadlock if those functions need to acquire a MetadataCtx
        let kvs = {
            let mdctx = ctx.md_ctx.get_or_init();

            let mut range = RangeQueryContext::new();
            if let Some(dir) = &mdctx.directory {
                range.set_directory(dir.clone());
            }
            if let Some(from) = &mdctx.from {
                range.set_from(from.clone());
            }
            // Using the KeyValues container makes displaying easy
            let mut req = ConfigReq::new(ctx)?;
            req.set_dir(range)?;
            KeyValues::from_model(req.get_all_pages()?)
        };

        match ctx.args.out_format {
            OutputFormat::Json => kvs.print_json(ctx)?,
            OutputFormat::Table => kvs.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.md_ctx.init(MetadataCtx::default());
        ctx.args.out_format = matches.value_of_t_or_exit("format");
        let mut mdctx = ctx.md_ctx.get_mut().unwrap();
        mdctx.base64 = matches.is_present("base64");
        mdctx.decode = matches.is_present("decode");
        mdctx.no_keys = matches.is_present("only-values");
        mdctx.no_values = matches.is_present("only-keys");
        mdctx.no_header = matches.is_present("no-header");
        mdctx.disp_encoding = matches.value_of_t_or_exit("display-encoding");
        mdctx.from =
            maybe_base64_arg!(matches, "from", matches.is_present("base64")).map(Key::from_encoded);
        mdctx.directory = maybe_base64_arg!(matches, "dir", matches.is_present("base64"))
            .map(Directory::from_encoded);
        Ok(())
    }
}
