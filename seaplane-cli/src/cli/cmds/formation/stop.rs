use clap::Parser;

use crate::{error::Result, Ctx};

/// Stop all instances of a Formation
#[derive(Parser)]
pub struct SeaplaneFormationStopArgs;

impl SeaplaneFormationStopArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationStopArgs::run")
    }
}
