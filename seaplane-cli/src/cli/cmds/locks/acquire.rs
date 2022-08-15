use clap::{ArgMatches, Command};

use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    ops::locks::HeldLock,
    printer::{Output, OutputFormat},
};

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane locks acquire` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
#[derive(Copy, Clone)]
pub struct SeaplaneLocksAcquire;

impl SeaplaneLocksAcquire {
    pub fn command() -> Command<'static> {
        Command::new("acquire")
            .visible_alias("acq")
            .about("Attempt to acquire the lock for N seconds")
            .arg(common::lock_name())
            .arg(common::ttl())
            .arg(common::base64())
            .arg(
                arg!(--("client-id") - ('L') =["STRING"] required).help(
                    "Client-chosen identifier stored with the lock for informational purposes",
                ),
            )
    }
}

impl CliCommand for SeaplaneLocksAcquire {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut req = LocksReq::new(ctx)?;
        let locksctx = ctx.locks_ctx.get_mut_or_init();
        let model_name = locksctx.lock_name.as_ref().map(|s| s.to_model());

        req.set_identifiers(model_name, locksctx.lock_id.as_ref().map(|s| s.encoded().to_owned()))?;

        let ttl = locksctx.ttl.as_ref().unwrap();
        let client_id: &str = locksctx.client_id.as_ref().unwrap();
        let held_lock_model = req.acquire(*ttl, client_id)?;

        let held_lock = HeldLock {
            lock_id: held_lock_model.id().encoded().to_owned(),
            sequencer: held_lock_model.sequencer(),
        };

        match ctx.args.out_format {
            OutputFormat::Json => held_lock.print_json(ctx)?,
            OutputFormat::Table => held_lock.print_table(ctx)?,
        }

        Ok(())
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.locks_ctx
            .init(LocksCtx::from_locks_common(&SeaplaneLocksCommonArgMatches(matches))?);

        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut locksctx = ctx.locks_ctx.get_mut().unwrap();
        locksctx.ttl = matches.get_one::<u32>("ttl").copied();
        locksctx.base64 = matches.contains_id("base64");
        locksctx.client_id = Some(matches.get_one::<String>("client-id").unwrap().to_string());

        Ok(())
    }
}
