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

#[cfg(any(feature = "ui_tests", feature = "semantic_ui_tests"))]
mod ui_tests {
    use super::Seaplane;
    use clap::{error::Error as ClapError, ArgMatches};
    use std::ffi::OsString;
    pub fn test_run<I, T>(argv: I) -> Result<ArgMatches, ClapError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Seaplane::command().try_get_matches_from(argv)
    }
}
#[cfg(any(feature = "ui_tests", feature = "semantic_ui_tests"))]
pub use ui_tests::test_run;
