#[macro_use]
mod macros;
mod cli;
mod config;
mod context;
mod data;
mod fs;
mod printer;

use std::{env, io::Write};

use anyhow::Result;
use clap::Parser;

use crate::{cli::SeaplaneArgs, config::RawConfig, context::Ctx, printer::OutputFormat};

static DEFAULT_LOG_VAR: &str = "SEAPLANE_LOG";

fn main() -> Result<()> {
    // Load a configuration file, we will check the raw values that can change aspects of the CLI.
    let _cfg = RawConfig::load()?;

    let args = SeaplaneArgs::parse();
    // Normally, this would be in the SeapalneArgs::run method, however setting up logging has to
    // happen super early in the process lifetime
    match args.verbose {
        0 => match args.quiet {
            0 => env::set_var(DEFAULT_LOG_VAR, "info"),
            1 => env::set_var(DEFAULT_LOG_VAR, "warn"),
            2 => env::set_var(DEFAULT_LOG_VAR, "error"),
            _ => env::set_var(DEFAULT_LOG_VAR, "off"),
        },
        1 => env::set_var(DEFAULT_LOG_VAR, "debug"),
        _ => env::set_var(DEFAULT_LOG_VAR, "trace"),
    }

    env_logger::Builder::from_env(DEFAULT_LOG_VAR)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let mut ctx = Ctx::from_config(&RawConfig::load()?)?;
    ctx.update_from_env()?;

    args.run(&mut ctx)
}
