use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    printer::OutputFormat,
};
use clap::{ArgMatches, Command};
use seaplane::api::v1::LockId;
use serde_json::json;

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLocksRelease;

impl SeaplaneLocksRelease {
    pub fn command() -> Command<'static> {
        Command::new("release")
            .visible_aliases(&["rel", "rl"])
            .override_usage("seaplane locks release <LOCK_NAME> --lock-id LOCK_ID [OPTIONS]")
            .about("Attempt to release a lock")
            .arg(common::lock_name())
            .arg(common::lock_id())
            .arg(common::base64())
    }
}

impl CliCommand for SeaplaneLocksRelease {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut req = LocksReq::new(ctx)?;
        let locksctx = ctx.locks_ctx.get_mut_or_init();
        req.set_identifiers(
            locksctx.lock_name.as_ref().map(|s| s.name.to_string()),
            locksctx.lock_id.as_ref().map(|s| s.encoded().to_owned()),
        )?;

        req.release()?;

        if ctx.args.out_format == OutputFormat::Table {
            cli_println!("Successfully released the lock");
        } else {
            cli_println!(
                "{}",
                json!({"name": ctx.locks_ctx.get_or_init().lock_name.as_ref().unwrap()})
            )
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.locks_ctx.init(LocksCtx::from_locks_common(
            &SeaplaneLocksCommonArgMatches(matches),
        )?);

        ctx.args.out_format = matches.value_of_t_or_exit("format");
        let mut locksctx = ctx.locks_ctx.get_mut().unwrap();
        locksctx.base64 = matches.is_present("base64");
        let raw_lock_id = matches.value_of("lock-id").unwrap();
        locksctx.lock_id = Some(LockId::from_encoded(raw_lock_id));

        Ok(())
    }
}
