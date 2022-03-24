use clap::{ArgMatches, Command};

use crate::{cli::CliCommand, context::Ctx, error::Result};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneConfig;

impl SeaplaneConfig {
    pub fn command() -> Command<'static> {
        todo!("SeaplaneConfig::into_app")
    }
}

impl CliCommand for SeaplaneConfig {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneConfig::run")
    }
    fn update_ctx(&self, _matches: &ArgMatches, _ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneConfig::update_ctx")
    }
}
