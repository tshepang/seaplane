use clap::Command;

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationConfiguration;

impl SeaplaneFormationConfiguration {
    pub fn command() -> Command<'static> {
        Command::new("configuration")
            .visible_alias("cfg")
            .about("Operate on Seaplane Formation Configurations")
    }
}

impl CliCommand for SeaplaneFormationConfiguration {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationConfiguration::run")
    }
}
