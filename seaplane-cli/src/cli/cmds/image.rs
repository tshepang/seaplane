use clap::Parser;

use crate::{context::Ctx, error::Result};

#[derive(Parser)]
pub struct SeaplaneImageArgs;

impl SeaplaneImageArgs {
    pub fn run(&self, _ctx: &Ctx) -> Result<()> {
        todo!("impl SeaplaneConfigArgs")
    }
}
