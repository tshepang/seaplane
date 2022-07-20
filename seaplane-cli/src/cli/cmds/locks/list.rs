use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::Result,
    ops::locks::{self, ListedLock},
    printer::OutputFormat,
};
use clap::{ArgMatches, Command};

static OUTPUT_PAGE_SIZE: usize = 10;

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

    let resp = req.get_lock_info()?;
    let out = ListedLock::from(resp);

    match ctx.args.out_format {
        OutputFormat::Json => cli_println!("{}", serde_json::to_string(&out)?),
        OutputFormat::Table => locks::print_lock_table(!locksctx.no_header, vec![out], ctx)?,
    };

    Ok(())
}

fn run_all_info(ctx: &mut Ctx) -> Result<()> {
    let mut last_key = None;
    let mut headers = !ctx.locks_ctx.get_or_init().no_header;
    let mut table_page = Vec::with_capacity(OUTPUT_PAGE_SIZE);

    loop {
        let mut req = LocksReq::new(ctx)?;
        let page = req.get_page(last_key)?;

        // We use the regular paging interface rather than
        // get_all_pages so that we don't have to store
        // all of the locks in memory at once.
        for info in page.infos {
            let out = ListedLock::from(info);
            match ctx.args.out_format {
                OutputFormat::Json => cli_println!("{}", serde_json::to_string(&out)?),
                OutputFormat::Table => {
                    table_page.push(out);
                    if table_page.len() >= OUTPUT_PAGE_SIZE {
                        locks::print_lock_table(headers, table_page.drain(..), ctx)?;
                        headers = false;
                    }
                }
            }
        }

        if let Some(next_key) = page.next {
            last_key = Some(next_key);
        } else {
            if !table_page.is_empty() {
                locks::print_lock_table(headers, table_page, ctx)?;
            }

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

        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut locksctx = ctx.locks_ctx.get_mut().unwrap();
        locksctx.base64 = matches.contains_id("base64");
        locksctx.decode = matches.contains_id("decode");
        locksctx.disp_encoding = matches
            .get_one("display-encoding")
            .copied()
            .unwrap_or_default();
        locksctx.no_header = matches.contains_id("no-header");

        Ok(())
    }
}
