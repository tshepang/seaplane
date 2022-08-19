use clap::{ArgMatches, Command};
use serde_json::json;

use crate::{
    api::MetadataReq,
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
        let mut req = MetadataReq::new(ctx)?;
        for kv in ctx.md_ctx.get_mut().unwrap().kvs.iter_mut() {
            let key = kv.key.to_string();
            req.set_key(key.clone())?;
            req.delete_value()?;
            if ctx.args.out_format == OutputFormat::Table {
                cli_println!("Removed {key}");
            }
            len += 1;
        }

        if ctx.args.out_format == OutputFormat::Table {
            cli_println!("\nSuccessfully removed {len} item{}", if len > 1 { "s" } else { "" });
        } else {
            cli_println!(
                "{}",
                json!({"removed": ctx.md_ctx.get_or_init().kvs.keys().map(|k| k.to_string()).collect::<Vec<_>>() })
            )
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.md_ctx
            .init(MetadataCtx::from_md_common(&SeaplaneMetadataCommonArgMatches(matches))?);
        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        Ok(())
    }
}
