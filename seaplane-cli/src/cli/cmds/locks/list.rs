use clap::{ArgMatches, Command};

use crate::{
    api::LocksReq,
    cli::cmds::locks::{common, common::SeaplaneLocksCommonArgMatches, CliCommand},
    context::{Ctx, LocksCtx},
    error::{CliError, CliErrorKind, Result},
    ops::locks::{self, ListedLock, LockName},
    printer::OutputFormat,
};

static OUTPUT_PAGE_SIZE: usize = 10;

static LONG_ABOUT: &str = "Get information around currently held locks.

There are 3 ways to list locks with this command:
- Omit the LOCK_NAME argument to list all locks
- Use a single lock name as the argument, without a trailing slash, this will list only that single lock
- Use a lock name followed by a trailing slash to list all locks under that directory

Locknames will be displayed in base64 encoded format by default because they may contain
arbitrary binary data. Using --decode to output the decoded values instead.";

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneLocksList;

impl SeaplaneLocksList {
    pub fn command() -> Command {
        Command::new("list")
            .visible_alias("ls")
            .about("Get information around currently held locks")
            .long_about(LONG_ABOUT)
            .arg(
                arg!(lock_name = ["LOCK_NAME"] !required)
                    .help("The name of a lock. If omitted, all locks are shown. Append a trailing slash to list directory contents"),
            )
            .arg(common::base64().requires("lock_name"))
            .args(common::display_args())
    }
}

fn run_one_info(ctx: &mut Ctx) -> Result<()> {
    let locksctx = ctx.locks_ctx.get_or_init();
    let lock_name = locksctx.lock_name.as_ref().unwrap();
    let model_name = lock_name.to_model();

    let mut req = LocksReq::new(ctx)?;
    req.set_name(model_name)?;

    let resp = req.get_lock_info()?;
    let out = ListedLock::from(resp);

    match ctx.args.out_format {
        OutputFormat::Json => cli_println!("{}", serde_json::to_string(&out)?),
        OutputFormat::Table => locks::print_lock_table(!locksctx.no_header, vec![out], ctx)?,
    };

    Ok(())
}

/// Looks up all held locks within this directory, using the root directory if `dir_name` is None.
fn run_dir_info(ctx: &mut Ctx, dir_name: Option<LockName>) -> Result<()> {
    let mut last_key = None;
    let dir = dir_name.map(|d| d.to_model());
    let mut headers = !ctx.locks_ctx.get_or_init().no_header;
    let mut table_page = Vec::with_capacity(OUTPUT_PAGE_SIZE);

    loop {
        let mut req = LocksReq::new(ctx)?;
        let page = req.get_page(last_key, dir.clone())?;

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

        // Check if the lock argument is root, directory or single lock
        match &locksctx.lock_name {
            // If there's no lock name given it's an "all locks" query
            None => run_dir_info(ctx, None),
            Some(name) => {
                // We need to at least peek at the final character of the decoded lock name to
                // determine if its a directory query or not.
                let mut decoded_lock_name = name
                    .name
                    .decoded()
                    .expect("decoding of a string we encoded shouldn't ever fail");

                if *decoded_lock_name
                    .last()
                    .expect("Lock name should hold something else it'd be None")
                    == b'/'
                {
                    // The SDK expects a lock name without the trailing slash for getting a
                    // directory, so we remove the `/`
                    decoded_lock_name.pop();
                    run_dir_info(ctx, Some(LockName::from_name_unencoded(decoded_lock_name)))
                } else {
                    run_one_info(ctx)
                }
            }
        }
    }

    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        ctx.locks_ctx
            .init(LocksCtx::from_locks_common(&SeaplaneLocksCommonArgMatches(matches))?);

        ctx.args.out_format = matches.get_one("format").copied().unwrap_or_default();
        let mut locksctx = ctx.locks_ctx.get_mut().unwrap();
        locksctx.base64 = matches.get_flag("base64");
        locksctx.decode = matches.get_flag("decode");
        locksctx.no_header = matches.get_flag("no-header");

        if locksctx.decode && ctx.args.out_format != OutputFormat::Table {
            let format_arg = format!("--format {}", ctx.args.out_format);
            return Err(CliError::from(CliErrorKind::ConflictingArguments(
                "--decode".to_owned(),
                format_arg,
            )));
        }

        Ok(())
    }
}
