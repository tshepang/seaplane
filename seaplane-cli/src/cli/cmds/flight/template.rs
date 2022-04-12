use clap::Command;

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightTemplate;

impl SeaplaneFlightTemplate {
    pub fn command() -> Command<'static> {
        Command::new("template")
    }
}

impl CliCommand for SeaplaneFlightTemplate {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFlightTemplate::run")
    }
}
