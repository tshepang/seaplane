use clap::{Arg, ArgMatches, Command};

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationContainerStatistics;

impl SeaplaneFormationContainerStatistics {
    pub fn command() -> Command<'static> {
        Command::new("container-statistics")
            .visible_alias(&["container-stats", "statistics", "stats"])
            .about("Display statistics about the underlying physical container instances");
        todo!("impl SeaplaneFormationContainerStatistics::command")
    }
}

impl CliCommand for SeaplaneFormationContainerStatistics {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationContainerStatistics::run")
    }
}
