use anyhow::Result;
use clap::Parser;

use crate::context::Ctx;

#[derive(Parser)]
pub struct SeaplaneImageArgs;

impl SeaplaneImageArgs {
    pub fn run(&self, _ctx: &Ctx) -> Result<()> {
        todo!("impl SeaplaneConfigArgs")
    }
}
