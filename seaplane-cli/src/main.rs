// Copyright ⓒ  2022 Seaplane IO, Inc.
// Licensed under the Apache 2.0 license
// (see LICENSE or <http://opensource.org/licenses/Apache-2.0>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

use clap::Parser;

use seaplane_cli::{
    cli::SeaplaneArgs, config::RawConfig, context::Ctx, error::Result, log::LogLevel,
    printer::OutputFormat,
};

fn try_main() -> Result<()> {
    let args = SeaplaneArgs::parse();
    // Normally, this would be in the SeapalneArgs::run method, however setting up logging has to
    // happen super early in the process lifetime
    match args.verbose {
        0 => match args.quiet {
            0 => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Info).unwrap(),
            1 => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Warn).unwrap(),
            2 => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Error).unwrap(),
            _ => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Off).unwrap(),
        },
        1 => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Debug).unwrap(),
        _ => seaplane_cli::log::LOG_LEVEL.set(LogLevel::Trace).unwrap(),
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
