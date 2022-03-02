use clap::Parser;

use crate::{context::Ctx, error::Result};

#[derive(Parser)]
pub struct SeaplaneConfigArgs;

impl SeaplaneConfigArgs {
    pub fn run(&self, _ctx: &Ctx) -> Result<()> {
        todo!("impl SeaplaneConfigArgs")
    }
}
