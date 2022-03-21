use clap::{Arg, ArgMatches, Command};

use crate::{cli::CliCommand, error::Result, Ctx};

#[derive(Copy, Clone, Debug)]
pub struct SeaplaneFormationLoadBalance;

impl SeaplaneFormationLoadBalance {
    pub fn command() -> Command<'static> {
        Command::new("load-balance")
            .visible_alias("lb")
            .about("Control how traffic balances between various configurations of a Formation")
    }
}

impl CliCommand for SeaplaneFormationLoadBalanceArgs {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationLoadBalance::run")
    }
}
