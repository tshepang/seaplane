use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    ops::locks::HeldLock,
    printer::{Output, OutputFormat},
};
use clap::{ArgMatches, Command};

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
            .override_usage(
                "seaplane locks acquire <LOCK_NAME> --client-id CLIENT_ID --ttl TTL [OPTIONS]",
            )
            .about("Attempt to acquire the lock")
            .arg(common::lock_name())
            .arg(common::ttl())
            .arg(common::base64())
            .arg(
                arg!(--("client-id") - ('L') =["CLIENT_ID"] required).help(
                    "Client-chosen identifier stored with the lock for informational purposes",
                ),
            )
    }
}

impl CliCommand for SeaplaneLocksAcquire {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let mut req = LocksReq::new(ctx)?;
        let locksctx = ctx.locks_ctx.get_mut_or_init();
        req.set_identifiers(
            locksctx.lock_name.as_ref().map(|s| s.name.to_string()),
            locksctx.lock_id.as_ref().map(|s| s.encoded().to_owned()),
        )?;

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
        ctx.locks_ctx.init(LocksCtx::from_locks_common(
            &SeaplaneLocksCommonArgMatches(matches),
        )?);

        ctx.args.out_format = matches.value_of_t_or_exit("format");
        let mut locksctx = ctx.locks_ctx.get_mut().unwrap();
        let raw_ttl = matches.value_of("ttl").unwrap();
        locksctx.ttl = Some(raw_ttl.parse().unwrap());
        locksctx.base64 = matches.is_present("base64");
        locksctx.client_id = Some(matches.value_of("client-id").unwrap().to_string());

        Ok(())
    }
}
