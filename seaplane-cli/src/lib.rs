// Copyright ⓒ  2022 Seaplane IO, Inc.
// Licensed under the Apache 2.0 license
// (see LICENSE or <http://opensource.org/licenses/Apache-2.0>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

#![warn(
    // TODO: we'll get to this
    //missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

#[macro_use]
pub mod macros;
pub mod api;
pub mod cli;
pub mod config;
pub mod context;
pub mod error;
pub mod fs;
pub mod log;
pub mod ops;
pub mod printer;

pub use crate::{
    cli::Seaplane, config::RawConfig, context::Ctx, error::Result, log::LogLevel,
    printer::OutputFormat,
};

#[cfg(any(feature = "ui_tests", feature = "semantic_ui_tests", feature = "api_tests"))]
mod tests {
    use std::ffi::OsString;

    use clap::ArgMatches;

    use crate::{
        cli::{CliCommand, Seaplane},
        context::Ctx,
        error::Result,
    };

    pub fn test_cli<I, T>(argv: I) -> Result<ArgMatches>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Seaplane::command().try_get_matches_from(argv)?;

        // Ensure we can grab the same args we do in normal main
        _ = matches.get_one::<u8>("verbose").copied();
        _ = matches.get_one::<u8>("quiet").copied();
        _ = matches.get_flag("stateless");
        Ok(matches)
    }

    pub fn test_main_update_ctx(matches: &ArgMatches) -> Result<()> {
        let mut ctx = Ctx::default();
        let s: Box<dyn CliCommand> = Box::new(Seaplane);
        s.traverse_update_ctx(matches, &mut ctx)?;
        Ok(())
    }

    pub fn test_main_exec_with_ctx(matches: &ArgMatches, mut ctx: Ctx) -> Result<()> {
        let s: Box<dyn CliCommand> = Box::new(Seaplane);
        s.traverse_exec(matches, &mut ctx)?;
        Ok(())
    }
}

#[cfg(any(feature = "ui_tests", feature = "semantic_ui_tests", feature = "api_tests"))]
pub use tests::{test_cli, test_main_exec_with_ctx, test_main_update_ctx};
