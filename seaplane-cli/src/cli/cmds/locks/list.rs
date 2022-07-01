use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    // ops::locks::LockName,
    printer::{Output, OutputFormat},
};
use clap::{ArgMatches, Command};

static LONG_ABOUT: &str = "Get information around a currently held lock.

Locknames will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values. However since they may contain arbitrary data, it's possible to re-encode them into a
different format for display purposes using --display-encoding";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLocksList;

impl SeaplaneLocksList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_aliases(&["ls", "l"])
            .override_usage("seaplane locks list <LOCK_NAME> [OPTIONS]")
            .about("Get information around a currently held lock")
            .long_about(LONG_ABOUT)
            .arg(common::lock_name())
            .arg(common::base64())
            .args(common::display_args())
    }
}

impl CliCommand for SeaplaneLocksList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let locksctx = ctx.locks_ctx.get_or_init();
        let mut req = LocksReq::new(ctx)?;

        if let Some(lock_name) = &locksctx.lock_name {
            req.set_name(lock_name.name.to_string())?;
        }
        req.get_lock_info()?;

        match ctx.args.out_format {
            OutputFormat::Json => ctx
                .locks_ctx
                .get_or_init()
                .lock_name
                .as_ref()
                .unwrap()
                .print_json(ctx)?,
            OutputFormat::Table => ctx
                .locks_ctx
                .get_or_init()
                .lock_name
                .as_ref()
                .unwrap()
                .print_table(ctx)?,
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
        locksctx.decode = matches.is_present("decode");
        locksctx.disp_encoding = matches.value_of_t_or_exit("display-encoding");
        locksctx.no_header = matches.is_present("no-header");

        Ok(())
    }
}
