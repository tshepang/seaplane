use clap::Command;

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationContainerStatistics;

impl SeaplaneFormationContainerStatistics {
    pub fn command() -> Command<'static> {
        Command::new("container-statistics")
            .visible_aliases(&["container-stats", "statistics", "stats"])
            .about("Display statistics about the underlying physical container instances")
    }
}

impl CliCommand for SeaplaneFormationContainerStatistics {
    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationContainerStatistics::run")
    }
}
