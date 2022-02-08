mod cli;
mod config;
mod context;

use anyhow::Result;
use clap::Parser;

use config::Config;
use context::Ctx;

fn main() -> Result<()> {
    #[cfg(feature = "color")]
    pretty_env_logger::init();
    #[cfg(not(feature = "color"))]
    env_logger::init();

    let args = cli::SeaplaneArgs::parse();

    // Load a configuration file
    let mut ctx = Ctx::from_config(&Config::load()?)?;
    ctx.update_from_env()?;

    args.run(&mut ctx)
}
