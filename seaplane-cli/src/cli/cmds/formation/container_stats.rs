use clap::Parser;

use crate::{error::Result, Ctx};

/// Display statistics about the underlying physical container instances
#[derive(Parser)]
#[clap(visible_alias = "container-stats")]
pub struct SeaplaneFormationContainerStatisticsArgs;

impl SeaplaneFormationContainerStatisticsArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationContainerStatisticsArgs")
    }
}
