use clap::Parser;

use crate::{error::Result, Ctx};

/// Control how traffic balances between formation configurations
#[derive(Parser)]
pub struct SeaplaneFormationTrafficConfigurationArgs;

impl SeaplaneFormationTrafficConfigurationArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationTrafficConfigurationArgs::run")
    }
}
