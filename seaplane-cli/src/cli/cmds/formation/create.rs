use clap::Parser;

use crate::{error::Result, Ctx};

/// Create a Seaplane Formation
#[derive(Parser)]
#[clap(visible_aliases = &["add"])]
pub struct SeaplaneFormationCreateArgs;

impl SeaplaneFormationCreateArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFormationCreateArgs")
    }
}
