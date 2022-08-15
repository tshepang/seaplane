use clap::Command;

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationTemplate;

impl SeaplaneFormationTemplate {
    pub fn command() -> Command<'static> {
        Command::new("template").about("Generate a template skeleton of a Formation")
    }
}

impl CliCommand for SeaplaneFormationTemplate {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> { todo!("impl SeaplaneFormationTemplate::run") }
}
