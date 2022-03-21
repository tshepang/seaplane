use clap::{Arg, ArgMatches, Command};

use crate::{context::Ctx, error::Result};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneConfig;

impl SeaplaneConfig {
    pub fn command() -> Command<'static> {
        todo!("SeaplaneConfig::into_app")
    }
}

impl CliCommand for SeaplaneConfig {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneConfig::run")
    }
    fn update_ctx(&self, matches: &ArgMatches, ctx: &mut Ctx) -> Result<()> {
        todo!("SeaplaneConfig::update_ctx")
    }
}
