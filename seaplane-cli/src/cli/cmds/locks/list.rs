use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    ops::locks::LockName,
    printer::{Output, OutputFormat},
};
use clap::{ArgMatches, Command};

static LONG_ABOUT: &str = "Get information around currently held locks.

Locknames will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode allows one to decode them and display the unencoded
values. However since they may contain arbitrary data, it's possible to re-encode them into a
different format for display purposes using --display-encoding";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLocksList;

impl SeaplaneLocksList {
    pub fn command() -> Command<'static> {
        Command::new("list")
            .visible_alias("ls")
            .about("Get information around currently held locks")
            .long_about(LONG_ABOUT)
            .arg(
                arg!(lock_name = ["LOCK_NAME"] !required)
                    .help("The name of a lock. If omitted, all locks are shown"),
            )
            .arg(common::base64().requires("lock_name"))
            .args(common::display_args())
    }
}

fn run_one_info(ctx: &mut Ctx) -> Result<()> {
    let locksctx = ctx.locks_ctx.get_or_init();
    let lock_name = locksctx.lock_name.as_ref().unwrap();

    let mut req = LocksReq::new(ctx)?;
    req.set_name(lock_name.name.to_string())?;
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

fn run_all_info(ctx: &mut Ctx) -> Result<()> {
    let mut last_key = None;
    loop {
        let mut req = LocksReq::new(ctx)?;
        let page = req.get_page(last_key)?;

        // We use the regular paging interface rather than
        // get_all_pages so that we don't have to store
        // all of the locks in memory at once.
        for info in page.infos {
            let nm = LockName::from_name(info.name.encoded());
            match ctx.args.out_format {
                OutputFormat::Json => nm.print_json(ctx)?,
                OutputFormat::Table => nm.print_table(ctx)?,
            }
        }

        if let Some(next_key) = page.next {
            last_key = Some(next_key);
        } else {
            break;
        }
    }

    Ok(())
}

impl CliCommand for SeaplaneLocksList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let locksctx = ctx.locks_ctx.get_or_init();
        if locksctx.lock_name.is_some() {
            run_one_info(ctx)
        } else {
            run_all_info(ctx)
        }
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
