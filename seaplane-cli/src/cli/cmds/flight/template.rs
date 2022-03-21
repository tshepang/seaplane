use clap::{Arg, ArgMatches, Command};

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFlightTemplate;

impl SeaplaneFlightTemplate {
    pub fn command() -> Command<'static> {
        Command::new("template").about("Generate a new template skeleton for a Flight definition");
        todo!("impl SeaplaneFlightTemplate::command")
    }
}

impl CLiCommand for SeaplaneFlightTemplateArgs {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFlightTemplate::run")
    }
}
