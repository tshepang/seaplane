mod cli;
mod config;
mod context;

use std::mem;

use anyhow::Result;
use clap::{AppSettings, FromArgMatches, IntoApp, Parser};

use cli::{SeaplaneArgs, SeaplaneDevArgs};
use config::RawConfig;

use context::Ctx;

fn main() -> Result<()> {
    #[cfg(feature = "color")]
    pretty_env_logger::init();
    #[cfg(not(feature = "color"))]
    env_logger::init();

    // Load a configuration file, we will check the raw values that can change aspects of the CLI.
    let cfg = RawConfig::load()?;

    // We first generate the clap::App representation from our struct definitions. This will allow
    // us to "enable" the `dev` command if the configuration file defines a `[dev]` section
    let mut app = cli::SeaplaneArgs::into_app();

    if cfg.dev.is_some() {
        if let Some(sc) = app.get_subcommands_mut().find(|sc| sc.get_name() == "dev") {
            // We replace the "hidden" command with a re-built command that is does not have that
            // Hidden setting set. We have to go through this memory dance because clap does not
            // provide a way to mutate a subcommand through a mutable reference, only through a move.
            let _ = mem::replace(
                sc,
                SeaplaneDevArgs::into_app()
                    .name("dev")
                    .unset_setting(AppSettings::Hidden),
            );
        }
    }

    let args = SeaplaneArgs::from_arg_matches(&app.get_matches())?;

    let mut ctx = Ctx::from_config(&cfg)?;

    ctx.update_from_env()?;

    args.run(&mut ctx)
}
