use clap::Parser;

use crate::{error::Result, Ctx};

/// Delete a Seaplane Formation
#[derive(Parser)]
#[clap(visible_aliases = &["del", "remove", "rm"])]
pub struct SeaplaneFormationDeleteArgs {
    /// Delete the formation even if launched
    #[clap(long)]
    force: bool,
}

impl SeaplaneFormationDeleteArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationDeleteArgs::run")
    }
}
