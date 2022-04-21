use clap::{ArgMatches, Command};
use serde_json::json;

use crate::{
    api::ConfigReq,
    cli::cmds::metadata::{common, common::SeaplaneMetadataCommonArgMatches, CliCommand},
    context::{Ctx, MetadataCtx},
    error::Result,
    printer::OutputFormat,
};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneMetadataDelete;

impl SeaplaneMetadataDelete {
    pub fn command() -> Command<'static> {
        Command::new("delete")
            .visible_aliases(&["del", "remove", "rm"])
            .override_usage("seaplane metadata delete <KEY>... [OPTIONS]")
            .about("Delete one or more metadata key-value pairs")
            .args(common::args())
    }
}

impl CliCommand for SeaplaneMetadataDelete {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut len = 0;
        let mut req = ConfigReq::new(ctx.args.api_key()?)?;
        #[cfg(feature = "api_tests")]
        {
            req.base_url(ctx.base_url.as_deref().unwrap());
        }
        for kv in ctx.md_ctx.get_mut().unwrap().kvs.iter_mut() {
            let key = kv.key.as_ref().unwrap().to_string();
            req.set_key(key.clone())?;
            req.delete_value()?;
            if ctx.args.out_format == OutputFormat::Table {
                cli_println!("Removed {key}");
            }
            len += 1;
        }

        if ctx.args.out_format == OutputFormat::Table {
            cli_println!(
                "\nSuccessfully removed {len} item{}",
                if len > 1 { "s" } else { "" }
            );
        } else {
            cli_println!(
                "{}",
                json!({"removed": ctx.md_ctx.get_or_init().kvs.keys().collect::<Vec<_>>() })
            )
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.md_ctx.init(MetadataCtx::from_md_common(
            &SeaplaneMetadataCommonArgMatches(matches),
        )?);
        ctx.args.out_format = matches.value_of_t_or_exit("format");
        Ok(())
    }
}
