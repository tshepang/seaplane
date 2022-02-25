use anyhow::Result;
use clap::Parser;

use crate::{
    Ctx,
};

/// Generate a new template skeleton for a Flight definition
#[derive(Parser)]
pub struct SeaplaneFlightTemplateArgs;

impl SeaplaneFlightTemplateArgs {
    pub fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        todo!("impl SeaplaneFlightTemplateArgs")
    }
}
