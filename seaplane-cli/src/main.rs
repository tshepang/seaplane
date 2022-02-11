mod cli;
mod config;
mod context;
mod dev;

use anyhow::Result;
use clap::{AppSettings, FromArgMatches, IntoApp, Parser};

use cli::{SeaplaneArgs, SeaplaneAccountArgs};
use config::RawConfig;

use context::Ctx;

fn main() -> Result<()> {
    #[cfg(feature = "color")]
    pretty_env_logger::init();
    #[cfg(not(feature = "color"))]
    env_logger::init();

    // Load a configuration file, we will check the raw values that can change aspects of the CLI.
    let cfg = RawConfig::load()?;

    let args = SeaplaneArgs::parse();

    let mut ctx = Ctx::from_config(&cfg)?;

    ctx.update_from_env()?;

    args.run(&mut ctx)
}
