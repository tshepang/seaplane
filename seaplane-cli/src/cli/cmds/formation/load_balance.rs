use clap::Parser;

use crate::{error::Result, Ctx};

/// Control how traffic balances between various configurations of a Formation
#[derive(Parser)]
pub struct SeaplaneFormationLoadBalanceArgs;

impl SeaplaneFormationLoadBalanceArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationLoadBalanceArgs::run")
    }
}
