#[macro_use]
mod macros;
mod cli;
mod config;
mod context;
mod error;
mod fs;
mod log;
mod ops;
mod printer;

use clap::Parser;

use crate::{
    cli::SeaplaneArgs, config::RawConfig, context::Ctx, error::Result, log::LogLevel,
    printer::OutputFormat,
};

fn try_main() -> Result<()> {
    let args = SeaplaneArgs::parse();
    // Normally, this would be in the SeapalneArgs::run method, however setting up logging has to
    // happen super early in the process lifetime
    match args.verbose {
        0 => match args.quiet {
            0 => crate::log::LOG_LEVEL.set(LogLevel::Info).unwrap(),
            1 => crate::log::LOG_LEVEL.set(LogLevel::Warn).unwrap(),
            2 => crate::log::LOG_LEVEL.set(LogLevel::Error).unwrap(),
            _ => crate::log::LOG_LEVEL.set(LogLevel::Off).unwrap(),
        },
        1 => crate::log::LOG_LEVEL.set(LogLevel::Debug).unwrap(),
        _ => crate::log::LOG_LEVEL.set(LogLevel::Trace).unwrap(),
    }

    let mut ctx = Ctx::from_config(&RawConfig::load_all()?)?;
    ctx.update_from_env()?;

    args.run(&mut ctx)
}

fn main() {
    if let Err(e) = try_main() {
        e.exit();
    }
}
