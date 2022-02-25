use anyhow::Result;
use clap::Parser;

use crate::context::Ctx;

#[derive(Parser)]
pub struct SeaplaneConfigArgs;

impl SeaplaneConfigArgs {
    pub fn run(&self, _ctx: &Ctx) -> Result<()> {
        todo!("impl SeaplaneConfigArgs")
    }
}
