use clap::Parser;

use crate::{error::Result, Ctx};

/// Operate on Seaplane Formation Configurations
#[derive(Parser)]
#[clap(visible_aliases = &["cfg"])]
pub struct SeaplaneFormationConfigurationArgs;

impl SeaplaneFormationConfigurationArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationConfigurationArgs")
    }
}
